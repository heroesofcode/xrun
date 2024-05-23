# Using

```sh
xrun extension project scheme version iPhone
```
### Example .xcodeproj
```sh
xrun project DeliveryApp.xcodeproj DeliveryApp 17.4 15
```

### Example .xcworkspace
```sh
xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15
```
### With fail

If you want when any test fails at the end, shows the terminal or CI as an error (it is optional if you don't use it and even test fails not to show it at terminal or CI as an error).

```sh
xrun workspace DeliveryApp.xcworkspace DeliveryApp 17.4 15 fail
```

<img src="https://github.com/heroesofcode/xrun/blob/main/img/example1.png" width="800" height="600">

- If all tests show no errors, a table will be displayed with the information.
<img src="https://github.com/heroesofcode/xrun/blob/main/img/example2.png" width="800" height="300">

- If all tests present errors, a table will be presented with the information and another table with only the errors.
<img src="https://github.com/heroesofcode/xrun/blob/main/img/example3.png" width="800" height="400">
