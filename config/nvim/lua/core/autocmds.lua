local dotfiles_group = vim.api.nvim_create_augroup("dotfiles", { clear = true })

vim.api.nvim_create_autocmd("TextYankPost", {
  group = dotfiles_group,
  desc = "Briefly highlight yanked text",
  callback = function()
    vim.highlight.on_yank({ timeout = 150 })
  end,
})
