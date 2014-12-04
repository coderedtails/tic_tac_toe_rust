# TicTacToe in Rust

This is a variation of [TicTacToe](http://en.wikipedia.org/wiki/Tic-tac-toe) written in [Rust](http://www.rust-lang.org/).

## Getting Started
Rust is a very young language which evolves on a weekly bases.
Hence, the current _rustc_ compiler may already have deprecated features.

The following version was used for development and testing:

```
$ rustc --version
rustc 0.13.0-nightly (fac5a0767 2014-11-26 22:37:06 +0000)
```

You can find an installer for Mac [here](https://www.dropbox.com/s/79xpgfkfhwt7rno/rust-nightly-x86_64-apple-darwin.pkg?dl=0).
Install it by simply double clicking the file and then run
`rustc --verison` from the command line.

The buildtol for Rust projects is [Cargo](https://github.com/rust-lang/cargo).
Cargo, like Rust, is still in early development.
The version used is:
```
$ cargo --version
cargo 0.0.1-pre-nightly (d6dbce7 2014-11-27 20:59:12 +0000)
```
You can find it [here](https://www.dropbox.com/s/gzhg05869ad7g6d/cargo-nightly-x86_64-apple-darwin.tar.gz?dl=0).

Building and testing the project is as easy changing into the checked out folder and running `cargo test` which will build and test the binary.
After a couple of seconds, a summary of the tests should be presented.

You can now start the game either using `cargo run` or changing into the `./target` directory and the running `./tic_tac_toe`.

## A Little Bit About Rust

Rust is a system language that tries to be *safe*, *fast* and *concurrent*.
Though you will not see any of the concurrent behaviour, you will _notice_ its speed both during the game itself and when the test run.

Most of all though, you will notice that Rust will go to great lengths to achieve compile-time *safety*.
You will notice this in the abundance of symbols and keywords in the source code that aim at being as explicit as possible to allow the compiler to do thorough checks

I will highlight some of these features to make it easier to undertand the code.
If you have C/C++ background you might recognize some of these features and syntax elements.

* Rust is statically typed but realies on type-inference. If the compile is not sure, the data type is placed between the variable name and its assignment:
    `let foo: uint = 12;`. For mutable access, add the _mut_ keyword: `let mut foo = 12u`
* Lines ending in semi colons are a statement (such as assignment) that *do not* return a value.
    Hence you'll see the last line in a function have no semi-colon, unless it return nothing.

* To ensure safety Rust forces you to declare _mutability_. This is a rather complex subject when talking about pointers, but you can think of it like lending a notepad to someone:
    ** If you only have immutable access, you can lend it to many other people, but only mutably.
    ** If you have mutable access, you can lend it someone else mutably, but he has to give it back to you before you give it back to its owner
* To guarantee these conditions, Rust introduces _lifetime bounds_ (`'a`), which declare that certain references have to be valid for a certain time.




