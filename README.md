<p align="center">
	<img src="https://raw.githubusercontent.com/heroesofcode/xrun/main/img/logo.png", width="530" height="230">
</p>

<p align="center">
		<a href="https://github.com/heroesofcode/xrun/actions/workflows/CI.yml"><img src="https://github.com/heroesofcode/xrun/actions/workflows/CI.yml/badge.svg"></a>
    <a href="https://crates.io/crates/xrun"><img src="https://img.shields.io/crates/v/xrun"></a>
		<a href="https://brew.sh/pt-br/"><img src="https://img.shields.io/badge/Homebrew-f1c072.svg?logo=homebrew&logoColor=7c6a50"></a>
    <a href="https://img.shields.io/badge/rustc-1.70.0-blue.svg?logo=rust&logoColor=orange"><img src="https://img.shields.io/badge/rustc-1.70.0-blue.svg?logo=rust&logoColor=orange"></a>
    <a href="https://crates.io/crates/xrun"><img src="https://img.shields.io/crates/d/xrun.svg?logo=rust&logoColor=orange"></a>
    <a href="https://github.com/heroesofcode/xrun/blob/main/LICENSE"><img src="https://img.shields.io/github/license/heroesofcode/xrun.svg"></a>
</p>

Command-line tools for macOS. With xrun you can run iOS and iPadOS unit tests through the terminal or CI with more ease and reading.

- [x] Shows the tests that were executed most easily.
- [x] Shows a table with test information.
- [x] If there is an error, it shows a table with failed tests.
- [x] Runs in the terminal and CI with an easy command

## Installing

### Cargo
Installing from [crates.io](https://crates.io/) (requires Rust/Cargo):

```shell
cargo install xrun
```

### Homebrew
You can install with [Homebrew](https://brew.sh/):

```shell
brew tap heroesofcode/taps
brew install heroesofcode/taps/xrun
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

#### Example with generate-file
If there are errors in the tests, use generate-file to generate a results-xrun.pdf file with the error table.

```sh
xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15 fail generate-file

or

xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15 generate-file
```

```
    __  __    ____      _   _   _   _
    \ \/"/ U |  _"\ uU |"|u| | | \ |"|
    /\  /\  \| |_) |/ \| |\| |<|  \| |>
   U /  \ u  |  _ <    | |_| |U| |\  |u
    /_/\_\   |_| \_\  <<\___/  |_| \_|
  ,-,>> \\_  //   \\_(__) )(   ||   \\,-.
   \_)  (__)(__)  (__)   (__)  (_")  (_/  (0.6.0)

💻 https://github.com/heroesofcode/xrun
===================================================

📋 Processing.......


CoreTests
    ✅ testSuccessWhenRegisteringTheUICollectionViewCellAndDequeuing (0.064 seconds)
    ✅ testSuccessWhenRegisteringTheUITableViewCellAndDequeuing (0.013 seconds)
    ✅ testWhenSetupBaseViewWithSuccess (0.001 seconds)

DesignSystemTests
    ✅ testShouldValidateLayout (0.137 seconds)

CoordinatorTests
    ✅ testHandleEvent (0.002 seconds)
    ✅ testInit (0.003 seconds)
    ✅ testStart (0.001 seconds)

AnalyticsTests
    ✅ testExample (0.001 seconds)

AuthenticationTests
    ✅ testShouldValidateLayout (0.093 seconds)


🗳️  The results have been completed below

+---------+-------------+-----------------+------------------+
| Runtime | Total Tests | ✅ Passed Tests |  ❌ Failed Tests |
+============================================================+
| 65.45s  | 9           | 9               | 0                |
+---------+-------------+-----------------+------------------+

👏 Congratulations, no errors were found!!!
```

If all tests present errors, a table will be presented with the information and another table with only the errors.

```
CoordinatorTests
    ✅ testHandleEvent (0.001 seconds)
    ❌ testInit, XCTAssertNil failed: "Coordinator.BaseCoordinator"
    ✅ testStart (0.000 seconds)

AnalyticsTests
    ✅ testExample (0.001 seconds)

AuthenticationTests
    ❌ testShouldValidateLayout, failed - Snapshot does not match reference.


🗳️  The results have been completed below

+---------+-------------+-----------------+------------------+
| Runtime | Total Tests | ✅ Passed Tests |  ❌ Failed Tests |
+============================================================+
| 35.64s  | 48          | 46              | 2                |
+---------+-------------+-----------------+------------------+

⚠️ Below contains the errors

+---------------------+------------------------------------------------------------------------------+
| Module              | Errors found                                                                 |
+====================================================================================================+
| CoordinatorTests    |     ❌ testInit, XCTAssertNil failed: "Coordinator.BaseCoordinator"          |
|---------------------+------------------------------------------------------------------------------|
| AuthenticationTests |     ❌ testShouldValidateLayout, failed - Snapshot does not match reference. |
+---------------------+------------------------------------------------------------------------------+
```

## GitHub Actions

It works on any CI, here I'll bring an example on GitHub Action for you to add to your iOS/iPadOS project

```yml
- name: Install xrun
  run: |
      brew tap heroesofcode/taps
      brew install heroesofcode/taps/xrun

- name: Run tests with xrun
  run: xrun project DeliveryApp.xcodeproj DeliveryApp 17.5 15 fail
```

## Contributing

To contribute, just fork this project and then open a pull request, feel free to contribute, bring ideas and raise any problem in the issue tab.

## License

xrun is released under the MIT license. See [LICENSE](https://github.com/heroesofcode/xrun/blob/main/LICENSE) for details.
