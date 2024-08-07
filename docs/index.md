<p align="center">
    <img src="https://raw.githubusercontent.com/heroesofcode/xrun/main/assets/logo.png" width="430" height="230" alt="Logo">
</p>

Command-line tools for macOS. With xrun you can run iOS and macOS unit tests through the terminal or CI with more ease and reading.

- Shows the tests that were executed most easily.
- Shows a table with test information.
- If there is an error, it shows a table with failed tests.
- It can run in the terminal and in a CI.
- Support for iOS and macOS.
- Generate PDF of failed tests.

```
    __  __    ____      _   _   _   _
    \ \/"/ U |  _"\ uU |"|u| | | \ |"|
    /\  /\  \| |_) |/ \| |\| |<|  \| |>
   U /  \ u  |  _ <    | |_| |U| |\  |u
    /_/\_\   |_| \_\  <<\___/  |_| \_|
  ,-,>> \\_  //   \\_(__) )(   ||   \\,-.
   \_)  (__)(__)  (__)   (__)  (_")  (_/  (0.10.0)

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
