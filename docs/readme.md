<p align="center">
	<img src="https://raw.githubusercontent.com/heroesofcode/xrun/main/img/logo.jpg" width="300" height="300">
</p>

<p align="center">
		<a href="https://github.com/heroesofcode/xrun/actions/workflows/CI.yml"><img src="https://github.com/heroesofcode/xrun/actions/workflows/CI.yml/badge.svg"></a>
    <a href="https://crates.io/crates/xrun"><img src="https://img.shields.io/crates/v/xrun"></a>
    <a href="https://img.shields.io/badge/msrv-1.70.0-blue.svg?logo=rust&logoColor=orange"><img src="https://img.shields.io/badge/msrv-1.70.0-blue.svg?logo=rust&logoColor=orange"></a>
    <a href="https://crates.io/crates/xrun"><img src="https://img.shields.io/crates/d/xrun.svg?logo=rust&logoColor=orange"></a>
    <a href="https://github.com/heroesofcode/xrun/blob/main/LICENSE"><img src="https://img.shields.io/github/license/heroesofcode/xrun.svg"></a>
</p>

Command-line tools for macOS. With xcrun you can run iOS and iPadOS unit tests through the CI terminal with more ease and reading.

- [x] Shows the tests that were executed most easily.
- [x] Shows a table with test information.
- [x] If there is an error, it shows a table with failed tests.
- [x] Runs in the terminal and CI with an easy command

## Installing
Installing from [crates.io](https://crates.io/) (requires Rust/Cargo):

```shell
cargo install xrun
```

## How to use?

```sh
xrun extension project scheme version iPhone
```

#### Example .xcodeproj
```sh
xrun project DeliveryApp.xcodeproj DeliveryApp 17.4 15
```

#### Example .xcworkspace
```sh
xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15
```

#### Example with fail
If you want when any test fails at the end, shows the terminal or CI as an error (it is optional if you don't use it and even test fails not to show it at terminal or CI as an error).

```sh
xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15 fail
```

<img src="https://raw.githubusercontent.com/heroesofcode/xrun/main/img/example1.png">

If all tests show no errors, a table will be displayed with the information.
<img src="https://raw.githubusercontent.com/heroesofcode/xrun/main/img/example2.png">

If all tests present errors, a table will be presented with the information and another table with only the errors.
<img src="https://raw.githubusercontent.com/heroesofcode/xrun/main/img/example3.png">

## GitHub Action

It works on any CI, here I'll bring an example on GitHub Action for you to add to your iOS/iPadOS project

```yml
- name: Install Rust
  uses: actions-rs/toolchain@v1
  with:
     toolchain: stable
     profile: minimal
     override: true

- name: Install xrun
  run: cargo install xrun

- name: Run tests with xrun
  run: xrun project DeliveryApp.xcodeproj DeliveryApp 17.2 15
```

## Changelog
- [Changelog](https://heroesofcode.github.io/xrun/changelog.html)

## Contributing

To contribute, just fork this project and then open a pull request, feel free to contribute, bring ideas and raise any problem in the issue tab.

## License

xrun is released under the MIT license. See [LICENSE](https://github.com/heroesofcode/xrun/blob/main/LICENSE) for details.
