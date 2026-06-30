local gh = function(repo)
  return ("https://github.com/%s.git"):format(repo)
end

vim.pack.add({
  { src = gh("folke/tokyonight.nvim") },
  { src = gh("folke/which-key.nvim") },
  { src = gh("hrsh7th/cmp-buffer") },
  { src = gh("hrsh7th/cmp-nvim-lsp") },
  { src = gh("hrsh7th/cmp-path") },
  { src = gh("hrsh7th/nvim-cmp") },
  { src = gh("L3MON4D3/LuaSnip") },
  { src = gh("mason-org/mason-lspconfig.nvim") },
  { src = gh("mason-org/mason.nvim") },
  { src = gh("neovim/nvim-lspconfig") },
  { src = gh("nvim-lua/plenary.nvim") },
  { src = gh("nvim-telescope/telescope.nvim") },
  { src = gh("nvim-tree/nvim-web-devicons") },
  { src = gh("nvim-lualine/lualine.nvim") },
  { src = gh("nvimdev/dashboard-nvim") },
  { src = gh("nvimdev/lspsaga.nvim") },
  { src = gh("numToStr/Comment.nvim") },
  { src = gh("rafamadriz/friendly-snippets") },
  { src = gh("ray-x/lsp_signature.nvim") },
  { src = gh("saadparwaiz1/cmp_luasnip") },
  { src = gh("stevearc/oil.nvim") },
  { src = gh("stevearc/overseer.nvim") },
})
