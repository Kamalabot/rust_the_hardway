
lvim.colorscheme = "default"
lvim.transparent_window = true
vim.o.relativenumber = true
lvim.format_on_save.enabled = true
lvim.format_on_save.pattern = { "*.py" }
-- format on save for rust files
vim.api.nvim_command("autocmd BufWritePre *.rs lua vim.lsp.buf.format()")

reload("user.options")
local formatters = require "lvim.lsp.null-ls.formatters"
local linters = require "lvim.lsp.null-ls.linters"

lvim.builtin.treesitter.ensure_installed = {
  "python",
  "lua",
  "rust",
  "toml"
}
-- setup debug adapter
lvim.builtin.dap.active = true
local mason_path = vim.fn.glob(vim.fn.stdpath "data" .. "/mason/")
pcall(function()
  require("dap-python").setup(mason_path .. "packages/debugpy/venv/bin/python")
end)

vim.list_extend(lvim.lsp.automatic_configuration.skipped_servers, { "rust_analyzer" })

local codelldb_path = mason_path .. "bin/codelldb"
local liblldb_path = mason_path .. "packages/codelldb/extension/lldb/lib/liblldb"
local this_os = vim.loop.os_uname().sysname

-- The path in windows is different
if this_os:find "Windows" then
  codelldb_path = mason_path .. "packages\\codelldb\\extension\\adapter\\codelldb.exe"
  liblldb_path = mason_path .. "packages\\codelldb\\extension\\lldb\\bin\\liblldb.dll"
else
  -- The liblldb extension is .so for linux and .dylib for macOS
  liblldb_path = liblldb_path .. (this_os == "Linux" and ".so" or ".dylib")
end

pcall(function()
  require("rust-tools").setup {
    tools = {
      executor = require("rust-tools/executors").termopen, -- can be quickfix or termopen
      reload_workspace_from_cargo_toml = true,
      runnables = {
        use_telescope = true,
      },
      inlay_hints = {
        auto = true,
        only_current_line = false,
        show_parameter_hints = true,
        parameter_hints_prefix = "<-",
        other_hints_prefix = "=>",
        max_len_align = false,
        max_len_align_padding = 1,
        right_align = false,
        right_align_padding = 7,
        highlight = "Comment",
      },
      hover_actions = {
        border = "rounded",
      },
      on_initialized = function()
        vim.api.nvim_create_autocmd({ "BufWritePost", "BufEnter", "CursorHold", "InsertLeave" }, {
          pattern = { "*.rs" },
          callback = function()
            local _, _ = pcall(vim.lsp.codelens.refresh)
          end,
        })
      end,
    },
    dap = {
      -- adapter= codelldb_adapter,
      adapter = require("rust-tools.dap").get_codelldb_adapter(codelldb_path, liblldb_path),
    },
    server = {
      on_attach = function(client, bufnr)
        require("lvim.lsp").common_on_attach(client, bufnr)
        local rt = require "rust-tools"
        vim.keymap.set("n", "K", rt.hover_actions.hover_actions, { buffer = bufnr })
      end,

      capabilities = require("lvim.lsp").common_capabilities(),
      settings = {
        ["rust-analyzer"] = {
          lens = {
            enable = true,
          },
          checkOnSave = {
            enable = true,
            command = "clippy",
          },
          -- helps automatic file formatting
          rustfmt = {
            enableRangeFormatting = true,
          }
        },
      },
    },
  }
end)

lvim.builtin.dap.on_config_done = function(dap)
  dap.adapters.codelldb = require("rust-tools.dap").get_codelldb_adapter(codelldb_path, liblldb_path)
  dap.configurations.rust = {
    {
      name = "Launch file",
      type = "codelldb",
      request = "launch",
      program = function()
        return vim.fn.input("Path to executable: ", vim.fn.getcwd() .. "/", "file")
      end,
      cwd = "${workspaceFolder}",
      stopOnEntry = false,
    },
  }
end

lvim.plugins = {
{
  "Pocco81/auto-save.nvim",
  config = function()
  require("auto-save").setup()
  end,
},
{
  url = "https://git.sr.ht/~whynothugo/lsp_lines.nvim",
  config = function()
  require("lsp_lines").setup()
  end,
},
  "simrat39/rust-tools.nvim",
  {
    "saecki/crates.nvim",
    tag = "stable",
    config = function()
    require("crates").setup()
    end,
  },
}

vim.keymap.set(
  "",
  "<Leader>f",
  require("lsp_lines").toggle,
  { desc = "Toggle lsp_lines" }
)
