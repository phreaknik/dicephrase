# dicephrase
A simple tool to help generate a mnemonic seed phrase using dice rolls as a source of entropy.

## Rust Dependencies
This is written in Rust and expects you to have a Rust toolchain installed. Install the latest Rust toolchain if you have not already. More info about Rust installation here: https://www.rust-lang.org/en-US/install.html
```
curl https://sh.rustup.rs -sSf | sh
```

## Build instructions
```
git clone git@github.com:phreaknik/dicephrase.git
cd dicephrase
cargo build
```

## Usage instructions
```
./target/debug/dicephrase help
```
or
```
./target/debug/dicephrase help [SUBCOMMAND]
```

## Example (Monero english dictionary)
```
./target/debug/dicephrase monero
```
