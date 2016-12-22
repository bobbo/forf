forf
====
**forf** is a utility for executing commands over sets of files. It takes a *glob*
and a *command string*, finds all the files matching the glob, then executes the
command string against them. It means you don't have to look up the archaic `for`
loop syntax for your shell whenever you want to run a command against a collection
of files. You can emulate most of this behaviour with `xargs`, but that wouldn't
have let me play with Rust.

### Examples
Prints contents of all Rust files in the source directory:
`$ forf "src/*.rs" "cat :f"`

## Installation
**forf** is built in Rust. Grab the Rust compile, clone this repo then run:

`$ cargo build --release`

Then copy `./target/release/forf` to somewhere on your `$PATH`.

## Usage
The general scheme is:

`forf "[glob]" [command string]`

The quotes around the glob stop your shell expanding the glob out for you.

#### Command string

The command strings are just standard commands (like you'd type into the terminal),
but there are some special expressions you can use to make your commands more powerful.
These expressions are expanded with specific values before the command is run.

- `:f` - The relative path of the current file

## License
* MIT license. See the [LICENSE](https://github.com/bobbo/forf/blob/master/LICENSE) file.
