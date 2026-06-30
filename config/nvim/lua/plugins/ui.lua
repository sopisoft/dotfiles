local has_tokyonight, tokyonight = pcall(require, "tokyonight")
if has_tokyonight then
  tokyonight.setup({
    style = "night",
  })
end

local has_lualine, lualine = pcall(require, "lualine")
if has_lualine then
  lualine.setup({
    options = {
      globalstatus = true,
      theme = "tokyonight",
    },
  })
end

local has_comment, comment = pcall(require, "Comment")
if has_comment then
  comment.setup()
end
