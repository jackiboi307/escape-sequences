# escape-sequences

Terminal escape sequences as constants in simple, nicely formatted files using string formatting syntax in different languages currently including:

- Python
- Rust
- Go

It includes both control and key sequences.

There are two versions of all files, split into two directories: `for_importing` and `for_copying`. The difference is mostly formatting.

## Usage

The strings are meant to be formatted using the standard formatting method in the file's language.

### Python

`CUR_SET.format(10, 20)`

### Go

`fmt.Sprintf(CUR_SET, 10, 20)`

You can also use `fmt.Printf` to print the result directly.

### Rust

While the strings use the same formatting syntax as the `format!` and related macros, they only accept literals and not constants, so you can't use them with the constants. I might make a function or macro to help with this, but otherwise you can format them like this:

`CUR_SET.replacen("{}", 10, 1).replacen("{}", 20, 1)`

## To do

- Add more modified keys (shift, control etc.)
- Add more control / style sequences
- Split control and key sequences into different files
- Add some convenient functions, including better formatting for Rust.

## Contributing

Feel free to add any missing sequences - this is done in the `for_importing/escape_sequences.py` file, which all the other files are generated from using `generate.py`. This means the formatting in `for_importing/escape_sequences.py` is important.

Also feel free to add any other language, which is done easily in the `generate.py` and `langs.py` files.

## Credits and helpful resources

- <https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797>
- <https://www.x.org/archive/X11R6.9.0/doc/PDF/ctlseqs.pdf>
- <https://www.gnu.org/software/screen/manual/screen.html#Control-Sequences>
- <https://aperiodic.net/pip/archives/Geekery/term-function-keys/>
