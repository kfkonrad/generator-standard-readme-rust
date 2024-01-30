#!/usr/bin/env bash

set -euf -o pipefail

ensure_targets() {
  required_targets=$*
  installed_targets=$(rustup target list --installed)
  for rt in $required_targets
  do
    found="no"
    for it in $installed_targets
    do
      if test "$it" = "$rt"
      then
        found="yes"
        break
      fi
    done
    if test "$found" = "no"
    then
      rustup target add "$rt"
    fi
  done
}

ensure_cross() {
  if ! which -s cross
  then
    cargo install cross --git https://github.com/cross-rs/cross
  fi
}

ensure_git_cliff() {
  if ! which -s git-cliff
  then
    cargo install git-cliff
  fi
}

check_gh() {
  if ! which -s gh
  then
    echo please install gh using one of the methods described in https://github.com/cli/cli#installation
    exit 1
  fi
}

ensure_cross
ensure_git_cliff
check_gh

version=$(grep '^version' < Cargo.toml | xargs | sed 's/"$//;s/.*"//')

macos="aarch64-apple-darwin x86_64-apple-darwin"
windows="x86_64-pc-windows-gnu"
linux="x86_64-unknown-linux-musl aarch64-unknown-linux-musl armv7-unknown-linux-musleabi armv7-unknown-linux-musleabihf"

# don't built macOS targets on non-Macs since macOS requires its SDK to be present for builds
if test "$(uname -s)" = "Darwin"
then
  ensure_targets "$macos"
  for target in $macos
  do
    echo cargo build --release --target "$target"
    cargo build --release --target "$target"
  done
fi


for target in $windows $linux
do
  echo cross build --release --target "$target"
  cross build --release --target "$target"
done

# now let's zip/tar.gz the bins
unix_like="$linux"
if test "$(uname -s)" = "Darwin"
then
  unix_like="$linux $macos"
fi

mkdir -p target/bins
for target in $unix_like
do
  tar -cZf "target/bins/${target}.tar.gz" -C "target/${target}/release" standard-readme
done

for target in $windows
do
  release_dir="target/${target}/release"
  pushd "$release_dir" > /dev/null
    zip -q "${target}.zip" standard-readme.exe
  popd > /dev/null
  mv "${release_dir}/${target}.zip" target/bins
done

git tag -- "$version"
git push --tags

changelog=$(mktemp)
git cliff --latest --strip all | sed 's/##/#/' > $changelog
gh release create "$version" -F $changelog ./target/bins/* --draft
rm $changelog
