# Ziqa Settings

The settings application for the Ziqa-IQ OS, based on the COSMIC desktop environment.

## Overview

Ziqa Settings is a modernized, user-friendly settings application designed for the Ziqa-IQ ecosystem. It provides a central hub for configuring system-wide preferences, hardware settings, and desktop appearance.

## Features

- **Appearance:** Customize themes, accent colors, and desktop wallpaper.
- **Networking:** Manage Wi-Fi, Ethernet, and Hotspot configurations.
- **System:** Handle system updates, date and time, and general system information.
- **Hardware:** Configure displays, input devices, and power management.

## Build

### Dependencies

See the `Build-Depends` section of the [debian control file](./debian/control).

### Install

This project uses [just](https://github.com/casey/just) as its build tool.

```sh
just
sudo just install
```

### Running

Run the settings app with:

```sh
just run
```

## License

Licensed under the [GNU Public License 3.0](https://choosealicense.com/licenses/gpl-3.0).
