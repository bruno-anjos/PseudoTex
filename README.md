# PseudoTex
> PseudoTex is a compiler that translates pseudocode into its LaTex representation

In university i had to do a few reports in LaTex that had pseudocode. Translating the simple pseudocode to the LaTex commands was a huge struggle so i decided to make this tool that translates a simple pseudocode language into the LaTex equivalent commands. I hope it makes you life easier :grin:.

The pseudocode that PseudoTex follows is somewhat formalized in this [paper](PseudoCodeNotes.pdf) by João Leitão (check [caveats](#Caveats)).

## Installing
For now there are two ways of installing PseudoTex, either by using `cargo` or by downloading the latest binary release.

### Using cargo

```shell
λ ~/ $ cargo install pseudotex
```

### Using latest release

```shell
λ ~/ $ wget https://github.com/bruno-anjos/PseudoTex
```

### Running

```shell
λ ~/ $ pseudotex --help
PseudoTex 0.1-alpha
Bruno Anjos <bruno.vale.anjos@gmail.com>
pseudocode transpiler to latex representation

USAGE:
    pseudotex [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      file name to read content from
    -o, --output <output>    file name to write content to
```

## Caveats

### Semi Colons

One main difference from the language examplified in the paper is that this every statement has to be followed by a semi colon. I believe this can be changed in the future, but since this made the parser easier to build i decided to take this approach.

## TODO

* Add more symbols
* Add tests
* Add test automation per commit
* Add subscript

## Contributing

If you'd like to contribute, please fork the repository and use a feature
branch. Pull requests are warmly welcome.

## Links

Project dependencies:
- [LALRPOP](http://lalrpop.github.io/lalrpop/)
- [clap](https://clap.rs)
- algorithm2e LaTex Package


## Licensing

The code in this project is licensed under GPL-3.0.
