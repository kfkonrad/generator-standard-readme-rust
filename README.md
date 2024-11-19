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
- [Publishing `standard-readme`](#publishing-standard-readme)
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

By default this will generate the README in English, other languages can be select by using
`standard-readme -l <LANGUAGE>`. Currently only English (`en`) and German (`de`) are supported.

## Publishing `standard-readme`

To publish a new version of `standard-readme` on crates.io you can run `cargo publish` like with any other Rust project
(assuming you have sufficient access to the crate).

To publish a new version of `standard-readme` as a GitHub release you can run `./release.sh` (again assuming you have
sufficient privileges). The `release.sh` script performs several steps:

1. Install `cross` and `git-cliff` if necessary
1. Check for the GitHub CLI `gh`
1. If running on macOS: Install the rustup targets `aarch64-apple-darwin` and `x86_64-apple-darwin` if necessary
1. Compile `standard-readme` for all configured targets. macOS binaries will only be created if run on a Mac. Linux
   and Windows binaries are cross-compiles with `cross`
1. Compress and collect the binaries for each platform
1. Create and push a git tag based on the version found in `Cargo.toml`
1. Create a GitHub release for that version. The changelog gets generated using `git-cliff` and the compressed binaries
   are uploaded automatically

`release.sh` will detect a dirty git state (such as unstaged or uncommitted changes) and skip creating a tag or release.

It also supports a dry-run mode that performs all the builds but skips the git tag and GitHub release steps as well. To
perform a dry run exectute `./release.sh --dry-run`. This is the only supported argument, all other arguments will be
ignored.

`release.sh` is compatible with both the BSD and GNU variants of all tools used. Other than the Rust toolchain only the
GitHub CLI `gh` needs to be installed for `release.sh` to be able to run.

## Maintainers

[@kfkonrad](https://github.com/kfkonrad)

## Contributing

PRs accepted.

Small note: If editing the README, please conform to the
[standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

MIT © 2024 Kevin F. Konrad
