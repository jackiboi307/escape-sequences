# escape-sequences

Terminal escape sequences in simple, nicely formatted files using string formatting syntax in different languages currently including:

- Python
- Rust
- Go

It includes both control codes and key codes. Key codes is a WIP though.

There are two versions of all files, split into two directories: `for_importing` and `for_copying`. The difference is mostly formatting.

Feel free to add any missing codes - this is done in the `for_importing/const.py` file, which all the other files are generated from using `generate.py`. This means the formatting in `for_importing/const.py` is important.

Also free to add any other language, which is done easily in the `generate.py` file.

I'd like to thank the creator of [this gist](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797) as I based this largely on it.
