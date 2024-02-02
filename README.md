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

You can download the correct version for your operating system and architecture using the `download.ps1` script. Don't
let the name fool you, the script works with Bash/ZSH on Linux or macOS too!

On Linux or macOS run:

```sh
curl -s https://raw.githubusercontent.com/kfkonrad/generator-standard-readme-rust/main/download.ps1 | bash
# OR
wget -qO- https://raw.githubusercontent.com/kfkonrad/generator-standard-readme-rust/main/download.ps1 | bash
```

On Windows run:

```pwsh
Invoke-Expression ((Invoke-WebRequest -Uri "https://raw.githubusercontent.com/kfkonrad/generator-standard-readme-rust/main/download.ps1").Content)
```

If you don't like running scripts from the internet you can find and download the application in the
[releases section of this repo](https://github.com/kfkonrad/generator-standard-readme-rust/releases) as well.

You can also install from source using cargo:

```sh
cargo install standard-readme
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
