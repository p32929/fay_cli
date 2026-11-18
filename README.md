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

## You might also like

<p><a href="https://openstore.pfay.workers.dev/more/p32929/fay_cli/0"><picture><source media="(prefers-color-scheme: dark)" srcset="https://openstore.pfay.workers.dev/more/p32929/fay_cli/0-dark.svg"><img src="https://openstore.pfay.workers.dev/more/p32929/fay_cli/0-light.svg" width="50%" align="top" alt="More apps by p32929 on OpenStore (1)"></picture></a><a href="https://openstore.pfay.workers.dev/more/p32929/fay_cli/1"><picture><source media="(prefers-color-scheme: dark)" srcset="https://openstore.pfay.workers.dev/more/p32929/fay_cli/1-dark.svg"><img src="https://openstore.pfay.workers.dev/more/p32929/fay_cli/1-light.svg" width="50%" align="top" alt="More apps by p32929 on OpenStore (2)"></picture></a><br><a href="https://openstore.pfay.workers.dev/more/p32929/fay_cli/2"><picture><source media="(prefers-color-scheme: dark)" srcset="https://openstore.pfay.workers.dev/more/p32929/fay_cli/2-dark.svg"><img src="https://openstore.pfay.workers.dev/more/p32929/fay_cli/2-light.svg" width="50%" align="top" alt="More apps by p32929 on OpenStore (3)"></picture></a><a href="https://openstore.pfay.workers.dev/more/p32929/fay_cli/3"><picture><source media="(prefers-color-scheme: dark)" srcset="https://openstore.pfay.workers.dev/more/p32929/fay_cli/3-dark.svg"><img src="https://openstore.pfay.workers.dev/more/p32929/fay_cli/3-light.svg" width="50%" align="top" alt="More apps by p32929 on OpenStore (4)"></picture></a></p>

<p><a href="https://p32929.github.io/share/?repo=fay_cli"><picture><source media="(prefers-color-scheme: dark)" srcset="https://p32929.github.io/readme/footer-share-dark.svg"><img src="https://p32929.github.io/readme/footer-share-light.svg" width="50%" align="top" alt="Share this project with your friends in one click"></picture></a><a href="https://p32929.github.io/support/"><picture><source media="(prefers-color-scheme: dark)" srcset="https://p32929.github.io/readme/footer-support-dark.svg"><img src="https://p32929.github.io/readme/footer-support-light.svg" width="50%" align="top" alt="If you like my work and want to support me, feel free to support"></picture></a><br><a href="https://openstore.pfay.workers.dev/add"><picture><source media="(prefers-color-scheme: dark)" srcset="https://p32929.github.io/readme/footer-join-dark.svg"><img src="https://p32929.github.io/readme/footer-join-light.svg" width="100%" align="top" alt="Put your own apps on OpenStore: free, your listing lives in your own repo"></picture></a></p>
