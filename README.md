# Cargo-prepublish

A Cargo plugin to help setup and prepare a Crate before publishing.

## How Does it work?

It reads your cargo.toml file and based on the values not present it will either set defaults or prompt you for input.

Some of the prompts will be required and others optional.

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


## Limitations

* Scattered array of tables (tables are reordered by default byt toml_edit) 
* ~~Similar to the previous, any comments will most likely be lost.~~ Toml_edit is now used and it retains the comments

## Contributions

Any contributions are welcome. If you find a bug, have a suggestion or feature request, please open a issue.
