# Spider Trap

## Inspiration

This project was inspired by [Spider Trap](https://github.com/adhdproject/spidertrap)

## Description

A simple tool for catching web crawlers.

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run
```

```bash
cargo run -- --port 8080 --webpages ./webpages.txt --min-links 5 --max-links 10 --min-length 3 --max-length 20
```

## Usage

```bash
Usage: spidertrap-rs [OPTIONS]

Options:
      --port <PORT>              [default: 8080]
      --webpages <WEBPAGES>
      --min-links <MIN_LINKS>    [default: 5]
      --max-links <MAX_LINKS>    [default: 10]
      --min-length <MIN_LENGTH>  [default: 3]
      --max-length <MAX_LENGTH>  [default: 20]
  -h, --help                     Print help
```
