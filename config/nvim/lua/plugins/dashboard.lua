local has_dashboard, dashboard = pcall(require, "dashboard")
if not has_dashboard then
  return
end

dashboard.setup({
  theme = "hyper",
  config = {
    week_header = {
      enable = true,
    },
    shortcut = {
      { desc = "Files", group = "Keyword", action = "Telescope find_files", key = "f" },
      { desc = "Grep", group = "DiagnosticHint", action = "Telescope live_grep", key = "g" },
      { desc = "Recent", group = "Number", action = "Telescope oldfiles", key = "r" },
      { desc = "Tasks", group = "DiagnosticWarn", action = "OverseerToggle", key = "t" },
    },
    project = {
      enable = false,
    },
    mru = {
      limit = 8,
    },
  },
})
