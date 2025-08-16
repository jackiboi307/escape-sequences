# escape-sequences

This project is my attempt at making terminal escape sequences less of a pain. It is not more than some .txt's in /data that via generate.py and the language configurations found in /langs are translated into files for the provided languages, that are easily imported or copied from.

I will add more sequences and languages when I have the need to, but feel free to do so yourself. The formats I have created are easy to understand and you probably won't have to dig into the not so pretty generate.py.

## Rust macros

The control sequences that accept arguments (`FG_RGB`, etc.) can not be used with the Rust formatter, despite using the same `{}` format. This is because Rust macros do not expand their arguments. Instead, you can use the below macros when necessary.

```rust
macro_rules! formatf {
    ($fmt:expr $(, $arg:expr)*) => {{
        let mut result = format!($fmt);
        $(
            result = result.replacen("{}", &$arg.to_string(), 1);
        )*
        result
    }};
}

macro_rules! printf {
    ($fmt:expr $(, $arg:expr)*) => {{
        print!("{}", formatf!($fmt $(, $arg)*));
    }};
}
```

Use them like this:

```
printf!(
    "{FG_RGB}{BG_RGB}Yellow text against a red background{RESET}\n",
    255, 255, 0,
    255, 0, 0
);
```

## Some useful stuff

- <https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797>
- <https://www.x.org/archive/X11R6.9.0/doc/PDF/ctlseqs.pdf>
- <https://www.gnu.org/software/screen/manual/screen.html#Control-Sequences>
- <https://aperiodic.net/pip/archives/Geekery/term-function-keys/>
