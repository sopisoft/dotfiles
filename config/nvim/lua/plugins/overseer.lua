local has_overseer, overseer = pcall(require, "overseer")
if not has_overseer then
  return
end

overseer.setup({
  strategy = "jobstart",
  task_list = {
    direction = "bottom",
    min_height = 12,
    max_height = 18,
    default_detail = 1,
  },
})

vim.keymap.set("n", "<leader>or", "<cmd>OverseerRun<CR>", { desc = "Run task", silent = true })
vim.keymap.set("n", "<leader>ot", "<cmd>OverseerToggle<CR>", { desc = "Toggle tasks", silent = true })
vim.keymap.set("n", "<leader>oo", "<cmd>OverseerQuickAction open float<CR>", { desc = "Task action", silent = true })
