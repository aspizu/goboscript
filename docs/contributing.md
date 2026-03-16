# Contributing

goboscript welcomes contributions in the form of Pull Requests. No LLM generated code
will be accepted.

goboscript is written in Rust. You'll need to install the
[Rust toolchain](https://www.rust-lang.org/tools/install) for development.

## Setup

Fork your own copy of the repository and clone it to your local machine.

```bash
git clone https://github.com/$USER/goboscript
cd goboscript
```

Install and set the default Rust toolchain to nightly:

```bash
rustup toolchain install nightly
rustup default nightly
```

## Development

To make development easier, and to validate the generated Scratch project -- use
the `tools/run.py` script:

```sh
tools/run.py --validate # or `-v`
```

This assumes that you have set-up a testing project at `playground/`. (You can create a
testing project by running `goboscript new -G playground`). It will compile the project,
validate it using the schemas from `scratch-parser`. If the validation fails, Scratch
will refuse to load the project.

To further debug the project, the generated `project.json` file can be extracted from
the `.sb3` file in the `playground/` directory, by running the following command:

```bash
tools/sb3.py playground/playground.sb3
```

Lets say that goboscript produced a broken project, and you are able to fix it by
hand -- by modifying the `project.json`.

You can add back the `project.json` to the `.sb3` file with:

```bash
# assuming that project.json is present at `playground/playground.json`
tools/sb3.py playground/playground.sb3 --patch # or `-p`
```

If you want to validate some `.sb3` file, use:

```bash
tools/sb3.py path/to/project.sb3 --validate # or `-v`
```
