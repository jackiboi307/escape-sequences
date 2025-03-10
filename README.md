# escape-sequences

Terminal escape sequences as constants in simple, nicely formatted files using string formatting syntax in different languages currently including:

- Python
- Rust
- Go

It includes both control and key sequences. Key sequences is a WIP though.

There are two versions of all files, split into two directories: `for_importing` and `for_copying`. The difference is mostly formatting.

## Usage

The strings are meant to be formatted using the preferred formatting in the file's language.

### Python

`CUR_SET.format(10, 20)`

### Go

`fmt.Sprintf(CUR_SET, 10, 20)`

You can also use `fmt.Printf` to print the result directly.

### Rust

The strings use the same formatting rules as the `format!` and related macros, but since they only accept literals and not constants, you can't use them with the constants. I might make a function or macro to help with this, but otherwise you can format them like this:

`CUR_SET.replacen("{}", 10, 1).replacen("{}", 20, 1)`

## To do

- Fix the key sequences, I based everything on my own terminal (Alacritty) but there are other standards I need to take care of.
- Split control and key sequences into different files.
- Move some stuff (the sequences and language formats) to JSON or some other format, instead of using Python for those purposes.
- Use RegEx instead of whatever I am doing now in `generate.py`.
- Add some convenient functions, including better formatting for Rust and a way to check for certain keys without handling the different standards yourself.

## Contributing

Feel free to add any missing sequences - this is done in the `for_importing/escape_sequences.py` file, which all the other files are generated from using `generate.py`. This means the formatting in `for_importing/escape_sequences.py` is important.

Also free to add any other language, which is done easily in the `generate.py` file.

## Credits and helpful resources

- <https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797>
