# goboscript

[![Discord](https://img.shields.io/discord/1216842627379363921?style=flat&logo=discord&label=Discord)](https://discord.gg/W9ZWy6ZMA3) ![image](https://shields.io/crates/l/goboscript) 

![](https://u.cubeupload.com/aspizu/Untitled202412111914.png)

[**Installation and documentation**](https://aspizu.github.io/goboscript)

goboscript is a text-based programming language that compiles to Scratch. Write
Scratch projects in text, and compile it into a `.sb3` file -- which can be opened
in the Scratch editor, TurboWarp or be uploaded to the Scratch website.

goboscript makes developing advanced Scratch projects FAST. goboscript syntax is
concise and easy to read. Use a version-control system such as git. Use VS Code
or your favourite text-editor. Share code by copy-pasting. Use the standard library.
Refactor code using search and replace. Write scripts in other programming languages
to generate goboscript code. goboscript allows you to integrate external tooling and
workflows, such as using a script to generate costumes for a text rendering engine, 
or loading in images into lists. goboscript has a powerful macro system similar to C.
The standard library includes many macros for frequently used patterns, such as
converting R, G, B values into a single integer. goboscript performs optimization,
removes unused code, and detects problems & mistakes.

goboscript is more than just an 1:1 mapping of Scratch blocks to text, it has
abstractions such as:

  - Custom data-types using Structs and Enums.
  - Functions that return values
  - Default parameters for Functions & Procedures
  - Operators such as: `!=`, `>=`, `<=`, `//` (Floor division), `not in`
  - Local variables (Function-scoped)
  - and more...

All these abstractions are compiled down to regular Scratch code.

### [Scratch Forum topic](https://scratch.mit.edu/discuss/topic/747370/)

## Sister Projects

 - [std](https://github.com/goboscript/std): The goboscript standard library.
 - [backpack](https://github.com/aspizu/backpack): Package manager for goboscript.
 - [sb2gs](https://github.com/aspizu/sb2gs): Decompile Scratch projects (.sb3) into goboscript projects (.gs)
 - [goboscript.ide](https://github.com/aspizu/goboscript.ide): Online IDE for goboscript, runs projects instantly in the browser.

### Other Text-Based Scratch projects

For a complete list of all text-based scratch projects, see <https://scratch.mit.edu/discuss/topic/792714/>

**@retr0id** first presented the demoscene discord with his `boiga` project (1). `boiga` works by
exporting Python data structures which nicely represent Scratch code in the form of
Python code. Soon after, I created my own re-implementation of `boiga` called `gobomatic`.
`gobomatic` was more feature-complete and supported more Scratch blocks and features than
`boiga` did, and it had some syntactical differences. The python version of `goboscript`
used `gobomatic` as a dependency to generate Scratch projects. Now, `gobomatic` is abandoned
and `goboscript` was ported to Rust.

(1): <https://github.com/DavidBuchanan314/boiga>

# Contributing

goboscript welcomes contributions in the form of Pull Requests.

goboscript is written in Rust. You'll need to install the [Rust toolchain](https://www.rust-lang.org/tools/install)
for development.

> [!NOTE]
> To install goboscript, follow instructions at [aspizu.github.io/goboscript](https://aspizu.github.io/goboscript).
> These instructions are for people who want to develop goboscript itself.

```sh
git clone https://github.com/aspizu/goboscript
cd goboscript
```

### Development

After cloning the repository, run goboscript locally from the repository root with:

```sh
cargo run -- build -i your_project/
```

But, to make development easier, and to validate the generated Scratch project - use
the `tools/run` script:

```sh
tools/run compile
```

This assumes that you have set-up a testing project at `playground/`.
It will compile the project, validate it using the schemas from `scratch-parser`.
If the validation fails, Scratch will refuse to load the project. To further debug
the project, the generated `project.json` file is extracted from the `.sb3` file in the
`playground/` directory.

Lets say that you modified the generated project in the Scratch editor or Turbowarp,
and you want to look at the `project.json`. You can extract it with:

```sh
tools/run uncompile
```

Lets say that goboscript produced a broken project, and you are able to fix it by hand -
by modifying the `project.json`. You can add back the `project.json` to the `.sb3` file
with:

```sh
tools/run patch
```

If you want to validate some `.sb3` file, use:

```sh
tools/run check path/to/project.sb3
```

### FOSS HACK 25

goboscript was one of the first-place winners of FOSS HACK 25, and was awarded a 50,000 Rs. prize.
FOSS HACK 25 was a open-source hackathon conducted on 22nd - 23rd February 2025 by the FOSS United
Foundation. During the 48-hour hackathon, I had worked on several goboscript issues and feature
implementation. Thank you FOSS United for the platform and opportunity.

## Star History

<a href="https://www.star-history.com/#aspizu/goboscript&Date">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=aspizu/goboscript&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=aspizu/goboscript&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=aspizu/goboscript&type=Date" />
 </picture>
</a>
