local has_which_key, which_key = pcall(require, "which-key")
if not has_which_key then
  return
end

which_key.setup({
  preset = "modern",
})

which_key.add({
  { "<leader>c", group = "code" },
  { "<leader>f", group = "find" },
  { "<leader>o", group = "overseer" },
  { "<leader>w", group = "workspace" },
})
