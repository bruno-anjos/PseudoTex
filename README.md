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
git clone https://github.com/bruno-anjos/PseudoTex.git
```

This command will create a directory named PseudoTex with the repository's content
in it.

### Running

For now since it is still in a very early stage, to run you need to use cargo.
(If you want to use a file you have to pipe it to stdin for now. I will add
[clap](https://clap.rs) for command arguments support)

```shell
cargo run < input_file
```

This will create a `demo.tex` file with the result.

## Features

PseudoTex supports:
* functions
* variables
* if
* if/else
* events
* state
* interface
    + requests
    + indications
* special symbols:
	+ in
	+ exists

## TODO

* Add [clap](https://clap.rs)
* Add if/else if
* Add if/else if/else
* Add triggers
* Add init
* Add procedures
* Add symbols
    + undefined value
* Add binaries to github
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
