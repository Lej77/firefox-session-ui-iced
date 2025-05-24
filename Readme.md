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
