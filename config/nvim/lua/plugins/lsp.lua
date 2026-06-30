local has_mason, mason = pcall(require, "mason")
if not has_mason then
  return
end

local has_mason_lspconfig, mason_lspconfig = pcall(require, "mason-lspconfig")
if not has_mason_lspconfig then
  return
end

local has_registry, registry = pcall(require, "mason-registry")
if not has_registry then
  return
end

local capabilities = vim.lsp.protocol.make_client_capabilities()

do
  local has_cmp_nvim_lsp, cmp_nvim_lsp = pcall(require, "cmp_nvim_lsp")
  if has_cmp_nvim_lsp then
    capabilities = cmp_nvim_lsp.default_capabilities(capabilities)
  end
end

do
  local has_lsp_signature, lsp_signature = pcall(require, "lsp_signature")
  if has_lsp_signature then
    lsp_signature.setup({
      bind = true,
      hint_enable = false,
      handler_opts = {
        border = "rounded",
      },
    })
  end
end

do
  local has_lspsaga, lspsaga = pcall(require, "lspsaga")
  if has_lspsaga then
    lspsaga.setup({
      lightbulb = {
        enable = false,
      },
      symbol_in_winbar = {
        enable = false,
      },
    })
  end
end

local server_settings = {
  lua_ls = {
    capabilities = capabilities,
    settings = {
      Lua = {
        diagnostics = {
          globals = { "vim" },
        },
        telemetry = {
          enable = false,
        },
        workspace = {
          checkThirdParty = false,
          library = vim.api.nvim_get_runtime_file("", true),
        },
      },
    },
  },
}

local preferred_servers = {
  bash = { "bashls" },
  c = { "clangd" },
  cmake = { "cmake" },
  cpp = { "clangd" },
  css = { "cssls" },
  dockerfile = { "dockerls" },
  go = { "gopls" },
  html = { "html" },
  javascript = { "vtsls", "ts_ls" },
  javascriptreact = { "vtsls", "ts_ls" },
  json = { "jsonls" },
  lua = { "lua_ls" },
  markdown = { "marksman" },
  nix = { "nil_ls" },
  python = { "basedpyright", "pyright" },
  rust = { "rust_analyzer" },
  sh = { "bashls" },
  toml = { "taplo" },
  typescript = { "vtsls", "ts_ls" },
  typescriptreact = { "vtsls", "ts_ls" },
  yaml = { "yamlls" },
  zsh = { "bashls" },
}

local filetype_fallbacks = {
  zsh = "sh",
}

local enabled_servers = {}
local installing_packages = {}
local registry_ready = false

local function contains(items, value)
  return vim.tbl_contains(items, value)
end

local function unique_list(items)
  local seen = {}
  local result = {}

  for _, item in ipairs(items) do
    if item ~= nil and not seen[item] then
      seen[item] = true
      table.insert(result, item)
    end
  end

  return result
end

local function enable_server(server_name)
  if enabled_servers[server_name] then
    return
  end

  local settings = server_settings[server_name]
  if settings ~= nil then
    vim.lsp.config(server_name, settings)
  else
    vim.lsp.config(server_name, {
      capabilities = capabilities,
    })
  end

  vim.lsp.enable(server_name)
  enabled_servers[server_name] = true
end

local function package_name_for(server_name)
  local mappings = mason_lspconfig.get_mappings()
  return mappings.lspconfig_to_package[server_name]
end

local function install_and_enable(server_name)
  local package_name = package_name_for(server_name)
  if package_name == nil or not registry.has_package(package_name) then
    enable_server(server_name)
    return
  end

  local package = registry.get_package(package_name)
  if package:is_installed() then
    enable_server(server_name)
    return
  end

  if installing_packages[package_name] then
    return
  end

  installing_packages[package_name] = true

  package:once("install:success", function()
    installing_packages[package_name] = nil
    vim.schedule(function()
      enable_server(server_name)
      vim.notify(("Installed %s via Mason"):format(server_name), vim.log.levels.INFO)
    end)
  end)

  package:once("install:failed", function()
    installing_packages[package_name] = nil
    vim.schedule(function()
      vim.notify(("Failed to install %s via Mason"):format(server_name), vim.log.levels.ERROR)
    end)
  end)

  vim.notify(("Installing %s via Mason"):format(server_name), vim.log.levels.INFO)
  package:install()
end

local function select_servers(filetype)
  local available_servers = mason_lspconfig.get_available_servers({ filetype = filetype })
  if #available_servers == 0 and filetype_fallbacks[filetype] ~= nil then
    available_servers = mason_lspconfig.get_available_servers({ filetype = filetype_fallbacks[filetype] })
  end
  local preferred = preferred_servers[filetype] or {}
  local selected = {}

  for _, server_name in ipairs(preferred) do
    if contains(available_servers, server_name) then
      table.insert(selected, server_name)
    end
  end

  if #selected == 0 and available_servers[1] ~= nil then
    table.insert(selected, available_servers[1])
  end

  return unique_list(selected)
end

local function maybe_enable_servers_for(filetype)
  if filetype == nil or filetype == "" then
    return
  end

  for _, server_name in ipairs(select_servers(filetype)) do
    install_and_enable(server_name)
  end
end

mason.setup({
  max_concurrent_installers = 1,
})

mason_lspconfig.setup({
  ensure_installed = { "lua_ls" },
  automatic_enable = false,
})

registry.refresh(vim.schedule_wrap(function(success)
  registry_ready = success
  if not success then
    vim.notify("Failed to refresh the Mason registry", vim.log.levels.WARN)
  end
end))

vim.diagnostic.config({
  severity_sort = true,
  underline = true,
  update_in_insert = false,
  virtual_text = {
    spacing = 2,
  },
})

vim.api.nvim_create_autocmd("FileType", {
  desc = "Install and enable an LSP server for the current filetype",
  callback = function(args)
    if not registry_ready then
      registry.refresh(vim.schedule_wrap(function(success)
        registry_ready = success
        if success then
          maybe_enable_servers_for(args.match)
        end
      end))
      return
    end

    maybe_enable_servers_for(args.match)
  end,
})

vim.api.nvim_create_autocmd("LspAttach", {
  desc = "Register buffer-local LSP keymaps",
  callback = function(args)
    local opts = { buffer = args.buf, silent = true }
    local has_saga = pcall(require, "lspsaga")

    if has_saga then
      vim.keymap.set("n", "gd", "<cmd>Lspsaga peek_definition<CR>", vim.tbl_extend("force", opts, { desc = "Peek definition" }))
      vim.keymap.set("n", "gr", "<cmd>Lspsaga finder<CR>", vim.tbl_extend("force", opts, { desc = "Find references" }))
      vim.keymap.set("n", "K", "<cmd>Lspsaga hover_doc<CR>", vim.tbl_extend("force", opts, { desc = "Hover documentation" }))
      vim.keymap.set("n", "<leader>ca", "<cmd>Lspsaga code_action<CR>", vim.tbl_extend("force", opts, { desc = "Code action" }))
      vim.keymap.set("n", "<leader>cr", "<cmd>Lspsaga rename<CR>", vim.tbl_extend("force", opts, { desc = "Rename symbol" }))
      vim.keymap.set("n", "<leader>co", "<cmd>Lspsaga outline<CR>", vim.tbl_extend("force", opts, { desc = "Outline" }))
      vim.keymap.set("n", "<leader>wd", "<cmd>Lspsaga show_line_diagnostics<CR>", vim.tbl_extend("force", opts, { desc = "Line diagnostics" }))
    else
      vim.keymap.set("n", "gr", vim.lsp.buf.references, vim.tbl_extend("force", opts, { desc = "List references" }))
      vim.keymap.set("n", "K", vim.lsp.buf.hover, vim.tbl_extend("force", opts, { desc = "Hover documentation" }))
      vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, vim.tbl_extend("force", opts, { desc = "Code action" }))
      vim.keymap.set("n", "<leader>cr", vim.lsp.buf.rename, vim.tbl_extend("force", opts, { desc = "Rename symbol" }))
      vim.keymap.set("n", "<leader>wd", vim.diagnostic.open_float, vim.tbl_extend("force", opts, { desc = "Line diagnostics" }))
    end

    vim.keymap.set("n", "gd", vim.lsp.buf.definition, vim.tbl_extend("force", opts, { desc = "Go to definition" }))
    vim.keymap.set("n", "gD", vim.lsp.buf.declaration, vim.tbl_extend("force", opts, { desc = "Go to declaration" }))
    vim.keymap.set("n", "gi", vim.lsp.buf.implementation, vim.tbl_extend("force", opts, { desc = "Go to implementation" }))
    vim.keymap.set("n", "<leader>cf", function()
      vim.lsp.buf.format({ async = true })
    end, vim.tbl_extend("force", opts, { desc = "Format buffer" }))
    vim.keymap.set("n", "<leader>wn", vim.diagnostic.goto_next, vim.tbl_extend("force", opts, { desc = "Next diagnostic" }))
    vim.keymap.set("n", "<leader>wp", vim.diagnostic.goto_prev, vim.tbl_extend("force", opts, { desc = "Previous diagnostic" }))
  end,
})
