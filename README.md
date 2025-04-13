# goboscript

![image](https://shields.io/crates/l/goboscript)

![](https://u.cubeupload.com/aspizu/Untitled202412111914.png)

[**Installation and documentation**](https://aspizu.github.io/goboscript)

goboscript is a text-based programming language which compiles to Scratch. It allows
you to write Scratch projects in text, and compile it into a .sb3 file - which can be
opened in the Scratch editor, TurboWarp or be uploaded to the Scratch website.

goboscript allows you to create advanced Scratch projects with ease, you can use any
text editor, use a version control system such as git. You can refactor your code
using search and replace. Text code can be copy pasted, which allows you to easily reuse
code or share it with others. goboscript syntax is concise and easy to read.

goboscript allows you to integrate external tooling and workflows, such as using a
script to generate costumes for a text rendering engine. Or loading in images into
lists.

goboscript has a powerful macro system - similar to Rust's macro system. This allows
you to write macros to generate code.

goboscript is more than just an 1:1 mapping of Scratch blocks to text, it also has
additional features like local variables for procedures (custom blocks).

goboscript also performs optimizations, detects problems and unused code.

# Sister Projects

### [**Package Manager**](https://github.com/aspizu/backpack)

### [**Decompiler**](https://github.com/aspizu/sb2gs)

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
implementation. Thank you FOSS United for the platform and oportunity. 

*This section will be updated with information about how the funds will be utilized.*
