vim.g.mapleader = " "
vim.g.maplocalleader = " "

vim.keymap.set("i", "jj", "<Esc>", { desc = "Exit insert mode", silent = true })
vim.keymap.set("n", "-", "<cmd>Oil<CR>", { desc = "Open parent directory", silent = true })
vim.keymap.set("n", "<leader>w", "<cmd>write<CR>", { desc = "Write buffer", silent = true })
vim.keymap.set("n", "<leader>q", "<cmd>quit<CR>", { desc = "Quit window", silent = true })
vim.keymap.set("n", "<leader>Q", "<cmd>qa!<CR>", { desc = "Quit all", silent = true })
