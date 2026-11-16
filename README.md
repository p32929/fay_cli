# fay

A small command line tool, written in Rust, that saves a series of shell commands under one name. Pick the name from a menu and fay runs every command in order.

If you keep typing the same setup, build or deploy steps one by one, fay remembers them for you. There are no flags to learn and no script files to write: you add the commands once in a simple menu, and the same tool works with bash or cmd on Windows, macOS and Linux.

![fay](https://user-images.githubusercontent.com/6418354/192144325-97d273db-3d74-4db8-bcec-62d0dfc86c72.gif)

## Features

1. **Save a list of commands** under a name, and run the whole list with one pick.
2. **Easy menu:** no flags to remember, just run `fay`.
3. **Works everywhere:** bash or cmd, on Windows, macOS and Linux.
4. **Easy to back up:** your saved commands are kept in a plain JSON file.

## Installation

```bash
cargo install fay
```

On [crates.io](https://crates.io/crates/fay).

## How to use

1. Install it with `cargo install fay`.
2. Type `fay` and press Enter.
3. Choose to add a command, give it a name and type the commands one by one.
4. Next time, run `fay` and pick that name from the list to run all of them.

## License

MIT License — Copyright (c) 2020 Fayaz Bin Salam. See [LICENSE](LICENSE) for the full text.

## Contributing

Contributions are very welcome, thank you! Before you start on a pull request, please open an issue and tell me what you would like to add or change. That way we can agree on it first, and your time is not wasted on something that might not fit. Thanks for helping make fay better.

## Share

<a href="https://p32929.github.io/share/?repo=fay_cli"><picture><source media="(prefers-color-scheme: dark)" srcset="https://p32929.github.io/readme/share-dark.svg"><img src="https://p32929.github.io/readme/share-light.svg" alt="Share this with your friends in just a click" width="420"></picture></a>

## Support

<a href="https://p32929.github.io/support/"><picture><source media="(prefers-color-scheme: dark)" srcset="https://p32929.github.io/readme/support-dark.svg"><img src="https://p32929.github.io/readme/support-light.svg" alt="If you like my works and want to support me, feel free to support" width="420"></picture></a>
