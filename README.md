# PseudoTex
> PseudoTex is a compiler that translates pseudocode into its LaTex representation

In university i had to do a few reports in LaTex that had pseudocode. Translating
the simple pseudocode to the LaTex commands was a huge struggle so i decided to make
this tool that translates a simple pseudocode language into the LaTex equivalent
commands. I hope it makes you life easier :grin:.

The pseudocode that PseudoTex follows is formalized in this [paper](PseudoCodeNotes.pdf)
by João Leitão.

## Installing

For now to install the tool you have to clone the repo and have rust and cargo
installed. The following command will create a directory with the repository
in you current folder.

```shell
λ ~/git/ $ git clone https://github.com/bruno-anjos/PseudoTex.git
```

This command will create a directory named PseudoTex with the repository's content
in it.

### Running

For now since it is still in a very early stage, to run you need to build the
project through cargo. There is a symbolic link that aims to the binary created in the
debug folder.

```shell
λ ~/git/PseudoTex/ $ cargo build
λ ~/git/PseudoTex/ $ ./pseudotex --help
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



## Features

PseudoTex uses [clap](https://clap.rs) for CLI argument parsing.

PseudoTex supports:
* functions
* variables
* if
* if/else
* events
* triggers
* state
* interface
    + requests
    + indications
* special symbols:
	+ in
	+ exists
    + set operators
    + not in
    + not exists
    + undefined value

## TODO

* Add if/else if
* Add if/else if/else
* Add init
* Add procedures
* Add setup timers
* Add setup periodic timers
* Add cancel timers
* Add cancel timers with args
* Add foreach
* Add tests
* Add test automation per commit

## Contributing

If you'd like to contribute, please fork the repository and use a feature
branch. Pull requests are warmly welcome.

## Links

Project dependencies:
- [LALRPOP](http://lalrpop.github.io/lalrpop/)


## Licensing

The code in this project is licensed under GPL-3.0.
