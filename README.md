# goboscript

[![image](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/W9ZWy6ZMA3)
![image](https://shields.io/crates/l/goboscript)

[**Installation and documentation**](https://aspizu.github.io/goboscript)

goboscript is the Scratch compiler. It lets you write Scratch projects in a text-based
programming language. It outputs `.sb3` files which can be opened in Scratch or
Turbowarp. It is written in Rust and is designed to have fast compile times.

goboscript is text-based, so you can use Visual Studio Code to write Scratch projects,
you can even use Github Copilot to help you write Scratch projects. goboscript also
performs optimizations, has a powerful macro system which lets you do meta-programming.
It even finds errors in your Scratch projects before you run them.


|goboscript code|Scratch blocks|
|---|---|
|![](https://media.discordapp.net/attachments/1216833762373931202/1217242019844591616/image.png?ex=66034ff8&is=65f0daf8&hm=adb07f66c0367119209da7d3c44b919766d1d6147a92d128401de23441efbf13&=&format=webp&quality=lossless&width=500&height=700)|![](https://media.discordapp.net/attachments/1216833762373931202/1217242200413442069/image.png?ex=66035023&is=65f0db23&hm=02de8a737307eac1c7c0d19b09a936549d9f179d2de2d38ef1be81dc7647e390&=&format=webp&quality=lossless&width=448&height=700)|

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
