## Introduction
---
Gdshader-lsp ins a language server for the Godot Shading Language that runs independently from the Godot engine. This allows you to edit gdshader files in your editor of choice. It is still a work in progress, but most of the key features of an LSP is included, including helpful error messages, hover hints, and code completion.
## Download Instructions
---
To download the server, run the following command in a terminal:
```
wget https://github.com/GodOfAvacyn/gdshader-lsp/releases/download/v0.1/gdshader-lsp
```
Alternatively, you can download the source code and build it yourself. This project was done in Rust, so you will need a variant of Cargo installed to use it.

There is currently very few ways to actually use this language server without additional work. I am in the process of writing a VsCode client extension for the server, and am trying to contact the maintainers of [lspconfig](https://github.com/neovim/nvim-lspconfig) for better Neovim support. For now, here is how to manually add the languag server to Neovim:
1. create a custom lua function somewhere in your neovim configuration:
   ```
    function gdshader()
      vim.lsp.start {
          name = "gdshader-lsp",
          cmd = {
              "<path to gdshader-lsp binary>",
          },
          capabilities = vim.lsp.protocol.make_client_capabilities()
      }
    end
   ```
2. When editing a .gdshader file, start the language server with ':lua gdshader()'

## Features
---
Gdshader-lsp currently has support for code completion, hover hints, and error messages. It lacks support for some key features - notably, any support for preprocessor macros (which, in its current form, this will probably be a deal-breaker for many people). Here is a full list of coming features that, in my opinion, would make it usable:
* Jump to definitioin
* Preprocessor macro support
* #include macros for .gdshaderinc files
* An actual Vscode extetnsion
* A spot among the supported lspconfig servers for Neovim.
