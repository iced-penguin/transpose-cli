# transpose-cli

## Overview

A command-line tool for transposing rows and columns from standard input.

## Requirements

- `rustc` and `cargo`

## Installation

```sh
cargo install transpose-cli
```

## Usage

The command-line tool is invoked using the `tp` command.

The default field delimiter is `,`. To specify a different field delimiter, use the `-d` or `--delimiter` option.

Example:
```sh
echo -e "a,b,c\nd,e,f" | tp
```

Output:
```
a,d
b,e
c,f
```