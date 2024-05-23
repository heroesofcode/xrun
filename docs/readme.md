# xrun

Command Line Tools for macOS. With xrun you can run Xcode unit tests faster through the terminal.

What is it for?

- Run all tests through the terminal
- At the end it shows a table with test information such as: Runtime, Total Tests, Passed Tests and Failed Tests.
- If the tests fail, it reports a table with only the errors.
- If you use CI, you can force it to fail if there are errors in the tests.

AnalyticsTests<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;❌ testExample, XCTAssertEqual failed: ("4") is not equal to ("2+3")

AuthenticationTests<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;✅ testShouldValidateLayout (0.068 seconds)

| Runtime | Total Tests | ✅ Passed Tests | ❌ Failed Tests |
| ------- | ----------- | -------------- | -------------- |
| 33.90s  | 48          | 47             | 1              |

| Errors found                                                        |
| ------------------------------------------------------------------- |
| ❌ testExample, XCTAssertEqual failed: ("4") is not equal to ("2+3") |

This tool was written in [Rust](https://www.rust-lang.org/)<br>
License we are using [MIT](https://github.com/heroesofcode/xrun/blob/main/LICENSE)

### More informations

- [Installing](https://heroesofcode.github.io/xrun/installing.html)
- [Using](https://heroesofcode.github.io/xrun/using.html)
- [Changelog](https://heroesofcode.github.io/xrun/changelog.html)
