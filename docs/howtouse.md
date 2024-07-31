To begin with you should use this order of arguments

```sh
// iOS
xrun extension project scheme version iPhone

// macOS
xrun extension project scheme macOS
```

### Example .xcodeproj
```sh
// iOS
xrun project DeliveryApp.xcodeproj DeliveryApp 17.4 15

// macOS
xrun project DeliveryApp.xcodeproj DeliveryApp macOS
```

### Example .xcworkspace
```sh
// iOS
xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15

// macOS
xrun workspace DeliveryApp.xcworkspace DeliveryApp macOS
```

### Example with fail
If you want when any test fails at the end, `shows the terminal or CI as an error` (it is optional if you don't use it and even test fails not to show it at terminal or CI as an error).

```sh
// iOS
xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15 fail

// macOS
xrun workspace DeliveryApp.xcworkspace DeliveryApp macOS fail
```

### Example generating pdf of the error
If there are errors in the tests, use generate-file to generate a `results-xrun.pdf` file with the error table.

```sh
// iOS
xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15 fail generate-file

or

xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15 generate-file

------------

// macOS
xrun workspace DeliveryApp.xcworkspace DeliveryApp macOS fail generate-file

or

xrun workspace DeliveryApp.xcworkspace DeliveryApp macOS generate-file
```
