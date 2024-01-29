# Standard Readme

[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

Generate Standard Readme-compatible READMEs

Standard Readme is a standard style for writing READMEs created by [@RichardLitt](https://github.com/RichardLitt). He
himself had implemented a Yeoman generator for creating standardized READMEs just like this one. This tool is
heavily inspired by [Richard's generator](https://github.com/RichardLitt/generator-standard-readme). I just wanted to
write an implementation that can be distributed as a single statically linked binary. This implementation supports every
feature the original one does and should generate near-identical READMEs.

See [the Standard Readme repo](https://github.com/RichardLitt/standard-readme) for more information on Standard Readmes.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Install

Right now the only way to install `standard-readme` is via `cargo`. Precompiled binaries will become available in the
future.

```sh
cargo install --git https://github.com/kfkonrad/generator-standard-readme-rust
```

## Usage

Simply run `standard-readme`:

```sh
standard-readme
```

You will be asked interactively for any additional information necessary to generate the README.

## Maintainers

[@kfkonrad](https://github.com/kfkonrad)

## Contributing

PRs accepted.

Small note: If editing the README, please conform to the
[standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

MIT © 2024 Kevin F. Konrad
