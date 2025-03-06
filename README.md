# escape-sequences

Terminal escape sequences as constants in simple, nicely formatted files using string formatting syntax in different languages currently including:

- Python
- Rust
- Go

It includes both control codes and key codes. Key codes is a WIP though.

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

## Contributing

Feel free to add any missing codes - this is done in the `for_importing/escape_sequences.py` file, which all the other files are generated from using `generate.py`. This means the formatting in `for_importing/escape_sequences.py` is important.

Also free to add any other language, which is done easily in the `generate.py` file.

## Credits

I'd like to thank the creator of [this gist](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797) as I based this largely on it.
