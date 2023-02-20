# Cargo-prepublish

[![crates.io](https://img.shields.io/crates/v/cargo-prepublish.svg)](https://crates.io/crates/cargo-prepublish)
[![docs](https://docs.rs/cargo-prepublish/badge.svg)](https://docs.rs/cargo-prepublish)


A Cargo plugin to help setup and prepare a Crate before publishing.

## How Does it work?

It reads your cargo.toml file and based on the values not present it will either set defaults or prompt you for input.

Some of the prompts will be required and others optional.

## Requirements

Git is required to be installed as tool uses the command line to run `git` commands.

## How to use

```sh
cargo install cargo-prepublish

# To make changes to the Cargo.toml file
cargo prepublish

# To make changes to the Cargo.toml file with just the defaults and not interactivity
cargo prepublish -n
# OR
cargo prepublish --non-interactive

# To check if the Cargo.toml is valid
cargo prepublish -v
# OR
cargo prepublish --valid
```

## What are the checks

It checks the following
* categories: If it doesn't exists then an empty array is added with a comment to the docs
* description
* documentation (Optional) Crates.io will link it to docs.rs automatically
* homepage: Uses git repo as the default if not set or entered
* keywords: At least one keyword needs to be entered. Default is the package name
* license and license-file: The default is license MIT OR Apache 2.0
* readme: Will first look for an existing one and if not, then one will be generated
* repository: Will extract for git if a valid git repo
* authors: Optional

### Extra checks

* package.metadata.docs.rs.all-features: This will only be applied for when a crate has features

## Limitations

* Scattered array of tables (tables are reordered by default by toml_edit) 

## Contributions

Any contributions are welcome. If you find a bug, have a suggestion or feature request, please open a issue.
