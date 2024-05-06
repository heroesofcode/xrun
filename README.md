# xrun

[![CI](https://github.com/heroesofcode/xrun/actions/workflows/CI.yml/badge.svg)](https://github.com/heroesofcode/xrun/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/cryptotools)](https://crates.io/crates/xrun)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/heroesofcode/xrun/blob/main/LICENSE)

Command Line Tools for macOS. With xrun you can run Xcode unit tests faster through the terminal.

## Installing
Installing from [crates.io](https://crates.io/) (requires Rust/Cargo):

```shell
cargo install xrun
```

## How to use?

```sh
xrun xcodeproj scheme version iPhone
```

```sh
xrun DeliveryApp.xcodeproj DeliveryApp 17.4 15
```

<img src="https://github.com/heroesofcode/xrun/blob/main/img/example1.png">

- If all tests show no errors, a table will be displayed with the information.
<img src="https://github.com/heroesofcode/xrun/blob/main/img/example2.png">

- If all tests present errors, a table will be presented with the information and another table with only the errors.
<img src="https://github.com/heroesofcode/xrun/blob/main/img/example3.png">

## Contributing

To contribute, just fork this project and then open a pull request, feel free to contribute, bring ideas and raise any problem in the issue tab.

## License

xrun is released under the MIT license. See [LICENSE](https://github.com/heroesofcode/xrun/blob/main/LICENSE) for details.
