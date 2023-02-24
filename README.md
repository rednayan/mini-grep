# mini-grep

This project is a minimal implementation of the grep command-line utility in Rust.
Installation

To use this program, you'll need to have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions on the official Rust website.

Once you have Rust and Cargo installed, you can clone this repository and run the following command in the project directory:

cargo build --release

This will build the program in release mode and create an executable file in the target/release/ directory.
Usage

The basic usage of the program is:

./target/release/minimal_grep <pattern> <file>

Where <pattern> is the regular expression pattern you want to search for, and <file> is the file you want to search in.

For example, if you want to search for the word "hello" in a file named myfile.txt, you can run:

./target/release/minimal_grep hello myfile.txt

By default, the program will print out all lines that match the pattern.

$IGNORE_CASE = 1 ./target/release/minimal_grep hello myfile.txt

By default, the program will run the query as case sensitive but can be made insensitive by setting the environment variable IGNORE_CASE to 1.

Contributing

If you'd like to contribute to this project, feel free to submit a pull request or open an issue. We welcome any suggestions, bug reports, or feature requests.
License
