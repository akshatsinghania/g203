# Logitech G203 user-space driver

Intention of this project is to write a user-space driver for Logitech G203 gaming mouse.

The projects consists of three parts:
 - Reverse engineer and document the USB protocol (which is based on USB-HID class)
 - Write `hidapi`-based library to communicate with the mouse
 - Write user interface


## References
 - [Logitech G203 Prodigy product page](https://www.logitechg.com/en-us/product/g203-prodigy-gaming-mouse)
 - [Logitech G500 code and documentation](https://github.com/cvuchener/g500)
 - [Device class definition for Human Interface Devices (HID)](http://www.usb.org/developers/hidpage/HID1_11.pdf)


## Protocol

`report` = A report as defined in USB-HID specification.

### LED modes on G203 mouse
The LED is controlled by setting report with ID `0x11`.

#### Color Cycle

```
ReportID|Device Index|Sub ID|Address|String                                                    |
      11|          ff|    0e|     3e|       0002|0000000000|       03e8|        64|000000000000|
        |            |      |       |Effect Type|     Zeros|Rate(speed)|Brightness|Zeros       |
```
```
FULL brightness, MIDDLE rate
11ff0e3e00020000000000271064000000000000
FULL  Brightness, FULL rate
11ff0e3e0002000000000003e864000000000000
FULL  Brightness, SLOW rate
11ff0e3e000200000000004e2064000000000000
MIDDLE  Brightness, SLOW rate
11ff0e3e000200000000004e2032000000000000##
ZERO  Brightness, SLOW rate
11ff0e3e000200000000004e2001000000000000
```

#### Breathing Effect

```
ReportID|Device Index|Sub ID|Address|String                                                     |
      11|          ff|    0e|     3e|       0003|ff0000|       03e8|00|        64|00000000000000|
        |            |      |       |Effect Type| R G B|Rate(speed)|00|Brightness|Zeros         |
```


```
FULL brightness, FULL rate, R=255, G=0, B=0
11ff0e3e0003ff000003e8006400000000000000
FULL brightness, FULL rate, R=0, G=255, B=0
11ff0e3e000300ff0003e8006400000000000000
FULL brightness, FULL rate, R=0, G=0, B=255
11ff0e3e00030000ff03e8006400000000000000
ZERO brightness, FULL rate, R=255, G=255, B=255
11ff0e3e0003ffffff03e8000100000000000000
ZERO brightness, SLOW rate, R=255, G=255, B=255
11ff0e3e0003ffffff4e20000100000000000000
```

#### Static light
```
ReportID|Device Index|Sub ID|Address|String                                           |
      11|          ff|    0e|     3e|       0001|ff0000|       02|00000000000000000000|
        |            |      |       |Effect Type| R G B|  Unknown|               Zeros|
```
##
```
R=255, G=255, B=255
11ff0e3e0001ffffff0200000000000000000000
```

#### Completely OFF

```
ReportID|Device Index|Sub ID|Address|String                                   |
      11|          ff|    0e|     3e|       0000|ffffff|0000000000000000000000|
        |            |      |       |Effect Type| R G B|                 Zeros|
```
```
11ff0e3e0000ffffff0000000000000000000000
```
