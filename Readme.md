# firefox-session-ui-iced

This is a graphical user interface for interacting with Firefox's session store
file that contains info about currently opened tabs and windows.

Note that this program simply makes use of the code exposed by the CLI tool at <https://github.com/Lej77/firefox_session_data>.

## Usage

- Build a release version locally using `cargo build --release` then run `target/release/firefox-session-ui-iced.exe`.
  - Or download a precompiled executable from the [latest GitHub release](https://github.com/Lej77/firefox-session-ui-iced/releases).
- When developing use: `cargo run`
- Build as website using [trunk](https://trunkrs.dev/) (`trunk serve` or `trunk build --release`),
  - You can try the current web demo at: <https://lej77.github.io/firefox-session-ui-iced/> (Note that currently it doesn't work yet, but you can see the appearance).

### `cargo install`

You can use `cargo install` to easily build from source without manually cloning the repo:

```bash
cargo install --git https://github.com/Lej77/firefox-session-ui-iced.git
```

You can use [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) to easily download the precompiled executables from a GitHub release:

```bash
cargo binstall --git https://github.com/Lej77/firefox-session-ui-iced.git firefox-session-ui-iced
```

After installing you can update the program using [nabijaczleweli/cargo-update: A cargo subcommand for checking and applying updates to installed executables](https://github.com/nabijaczleweli/cargo-update):

```bash
cargo install-update --git firefox-session-ui-iced

# OR update all installed programs:
cargo install-update --git --all
```

You can uninstall uisng:

```bash
 cargo uninstall firefox-session-ui-iced
```

## License

This project is released under either:

- [MIT License](./LICENSE-MIT)
- [Apache License (Version 2.0)](./LICENSE-APACHE)

at your choosing.

Note that some optional dependencies might be under different licenses.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
