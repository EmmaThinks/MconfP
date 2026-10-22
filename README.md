# MconfP

This project is meant to manage your monitors from terminal without the need of a visual interface.

## ! Disclaimer
There's a few things to consider with this project, it is not meant to replace any other project or tool, i simply made this rust program (because at this point, script is not fitting for what it actually is) to learn. There's a lot to improve (specially support for laptops and optimizations). But i think it's already a good starting point in case anyone wants to fork it and make its own version or give me feedback about what could i improve to make this script even better.

This script also needs the kernel module "i2c" to be loaded (`sudo modprobe i2c-dev`) your user also needs to be in the i2c group (but you can also use `sudo` if you don't want or can't add your user to i2c group).

## Features and capabilities

- Monitor management via i2c and "/sys/class/backlight" for laptops
- Threading for each flag that modifies system files.
- Monitor targeting via `-t` `--target` flag with id or name. (please do not try to target integrated monitors with id, there's an "int" keyword for integrated monitors)

## Available commands
`--show-displays | -sd`                          Display the name of detected Monitors and their id.

`--change-brightness:<value> | -cb:<value>`      Change the brightness of selected Monitors.

`--target:<monitor> | -t:<monitor>`              Target specific monitors via id or name.

`--turn-off-monitors | -turnOff`                 Turn off selected Monitors (only external).

`--help | -h`                                    Display help message.
