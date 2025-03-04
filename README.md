# escape-codes

Terminal escape codes in simple, nicely formatted files using string formatting syntax in different languages currently including:

- Python
- Rust
- Go

It includes both control codes and key codes like ESCAPE. Key codes is a WIP though.

There are also a few handy "shortcuts", like CURSOR\_DOWN\_ONE which is simply CURSOR\_DOWN with the formatting set to "1", which is a common value.

Feel free to add any missing codes - this is done in the `const.py` file, which all the other files are generated from using `generate.py`. This means the formatting in `const.py` is important.

Also free to add any other language - that is done easily in the `generate.py` file.

The `generate.py` script includes the ability to toggle "even spacing", meaning that each row will include additional spacing to make the codes appear on the same column. This makes it easier to read, but it is less useful for copy-pasting. In the future, I'll add a folder in the const\_files directory where this option is toggled off or on.

I'd like to thank the creator of [this gist](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797) as I based this largely on it.
