<p align="center">
	<img src="https://github.com/heroesofcode/xrun/blob/main/img/logo.jpg" width="300" height="300">
</p>

<p align="center">
		<a href="https://github.com/heroesofcode/xrun/actions/workflows/CI.yml"><img src="https://github.com/heroesofcode/xrun/actions/workflows/CI.yml/badge.svg"></a>
    <a href="https://crates.io/crates/xrun"><img src="https://img.shields.io/crates/v/xrun"></a>
    <a href="https://img.shields.io/badge/msrv-1.70.0-blue.svg?logo=rust&logoColor=orange"><img src="https://img.shields.io/badge/msrv-1.70.0-blue.svg?logo=rust&logoColor=orange"></a>
    <a href="https://crates.io/crates/xrun"><img src="https://img.shields.io/crates/d/xrun.svg?logo=rust&logoColor=orange"></a>
    <a href="https://github.com/heroesofcode/xrun/blob/main/LICENSE"><img src="https://img.shields.io/github/license/heroesofcode/xrun.svg"></a>
</p>

Command Line Tools for macOS. With xrun you can run Xcode unit tests faster through the terminal.

## Installing
Installing from [crates.io](https://crates.io/) (requires Rust/Cargo):

```shell
cargo install xrun
```

## How to use?

```sh
xrun extension project scheme version iPhone
```
![Custom badge](https://img.shields.io/badge/-EXAMPLE%20.XCODEPROJ-orange?style=for-the-badge)

```sh
xrun project DeliveryApp.xcodeproj DeliveryApp 17.4 15
```

![Custom badge](https://img.shields.io/badge/-EXAMPLE%20.XCWORKSPACE-orange?style=for-the-badge)
```sh
xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15
```

![Custom badge](https://img.shields.io/badge/-WITH%20FAIL-orange?style=for-the-badge)

If you want when any test fails at the end, shows the terminal or CI as an error (it is optional if you don't use it and even test fails not to show it at terminal or CI as an error).

```sh
xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15 fail
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
