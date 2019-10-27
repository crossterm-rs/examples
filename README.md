![Lines of Code][s7] [![MIT][s2]][l2] [![Join us on Discord][s5]][l5]

# Crossterm Examples

The examples are compatible with the latest release. When you work with the master branch, checkout the pull request list for up-to-date examples. 

## Structure

```
├── examples
│   └── src
│       └── bin
├── first-depth-search
└── snake
```

* `examples/src/bin` - various examples demonstrating the [`crossterm`](https://crates.io/crates/crossterm) crate
  features.
* `first-depth-search` - [Depth-first search](https://en.wikipedia.org/wiki/Depth-first_search) example.
* `snake` - snake game.

## Run examples

```bash
$ cargo run --bin snake
$ cargo run --bin first-depth-search
$ cargo run --bin alternate_screen
...
```

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project is licensed under the MIT License - see the [LICENSE.md](./LICENSE) file for details.

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ./LICENSE

[s5]: https://img.shields.io/discord/560857607196377088.svg?logo=discord
[l5]: https://discord.gg/K4nyTDB

[s7]: https://travis-ci.org/crossterm-rs/examples.svg?branch=master
