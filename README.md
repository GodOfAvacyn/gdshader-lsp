## Introduction
---
Gdshader-lsp ins a language server for the Godot Shading Language that runs independently from the Godot engine. This allows you to edit gdshader files in your editor of choice. It is still a work in progress, but most of the key features of an LSP is included, including helpful error messages, hover hints, and code completion.

This was made at the same time as [tree-sitter-gdshader](https://github.com/GodOfAvacyn/tree-sitter-gdshader), which is used for syntax highlighting (also a WIP, but support for neovim is sort of there).
## Download Instructions
---
GDShader is now available as a VSCode extension! Getting it that way comes built-in with syntax highlighting, so you don't need to do any nonsense with my treesitter repo.

To download the server for manual use, run the following command in a terminal to grab the binary:
```
wget https://github.com/GodOfAvacyn/gdshader-lsp/releases/download/v0.1/gdshader-lsp
```
Alternatively, you can download the source code and build it yourself. This project was done in Rust, so you will need a variant of Cargo installed to use it.

If you are a neovim user, here is how to manually add the languag server to Neovim:
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
2. When editing a .gdshader file, start the language server with ':lua gdshader()'. You'll need to call that function for each new gdshader file you open (but only once) (and until I can set up a real client for neovim).
3. (Optional) follow steps at [tree-sitter-gdshader](https://github.com/GodOfAvacyn/tree-sitter-gdshader) to get syntax highlighting support.

## Features
---
Gdshader-lsp currently has support for code completion, hover hints, error messages, and include statements. It lacks support for some key features - notably, support for other preprocessor macros (which, in its current form, this will probably be a deal-breaker for many people). Here is a full list of coming features that, in my opinion, would make it more usable, in my opinion:
* Jump to definitioin
* Preprocessor macro support
* A spot among the supported lspconfig servers for Neovim.
