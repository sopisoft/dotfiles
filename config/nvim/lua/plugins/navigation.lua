local has_telescope, telescope = pcall(require, "telescope")
if has_telescope then
  telescope.setup({
    defaults = {
      layout_config = {
        prompt_position = "top",
      },
      path_display = { "smart" },
      sorting_strategy = "ascending",
    },
    pickers = {
      find_files = {
        hidden = true,
      },
    },
  })

  local builtin = require("telescope.builtin")

  vim.keymap.set("n", "<leader>ff", builtin.find_files, { desc = "Find files", silent = true })
  vim.keymap.set("n", "<leader>fg", builtin.live_grep, { desc = "Live grep", silent = true })
  vim.keymap.set("n", "<leader>fb", builtin.buffers, { desc = "Buffers", silent = true })
  vim.keymap.set("n", "<leader>fh", builtin.help_tags, { desc = "Help tags", silent = true })
  vim.keymap.set("n", "<leader>fr", builtin.oldfiles, { desc = "Recent files", silent = true })
end

local has_oil, oil = pcall(require, "oil")
if not has_oil then
  return
end

oil.setup({
  columns = {
    "icon",
    "permissions",
    "size",
    "mtime",
  },
  default_file_explorer = true,
  delete_to_trash = true,
  skip_confirm_for_simple_edits = true,
  view_options = {
    show_hidden = true,
  },
})

vim.keymap.set("n", "<leader>e", "<cmd>Oil<CR>", { desc = "Explorer", silent = true })
