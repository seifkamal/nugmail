# Nugmail

A command line client for generating disposable email addresses.

## Installation

### Cargo

```shell script
cargo install --git https://github.com/seifkamal/nugmail
```

## Usage

```
> nugmail help
Nugmail 0.1.0
A client for generating disposable email addresses

USAGE:
    nugmail <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    delete    Delete an address and its inbox
    help      Prints this message or the help of the given subcommand(s)
    inbox     View an address inbox
    list      Display all generated addresses
    new       Generate a new address
```

This is currently a **WIP**; Some known issues are:
- An inbox cannot be synced whilst in view (must exit and reenter)
- Email messages are rendered using `less` (see
[f226192](https://github.com/seifkamal/nugmail/commit/f22619233eef6bfd3b65653cff95553789736b02))
- Date values from email client are not converted to user timezone
- Sending emails is not supported
- Downloading attachments is not supported
