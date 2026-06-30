require("core.pack")

vim.cmd("filetype plugin indent on")
vim.cmd("syntax on")

require("core.options")
require("core.keymaps")
require("core.autocmds")
require("plugins")

pcall(vim.cmd.colorscheme, "tokyonight-night")
