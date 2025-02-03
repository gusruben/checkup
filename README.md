<div align="center">
    <img height="300" src="https://github.com/gusruben/checkup/raw/main/checkup.png">

<h1 align="center"></h1>
</div>

Checkup is a simple tool for detecting changes on websites, and running corresponding actions. It's easy to build Checkup into shell pipelines, home automation, or anything else that can be accessed from the terminal. Get creative-- play sounds, open programs, flash lights, automate the world!

## Usage

```
Usage: checkup.exe [OPTIONS] <URL> <COMMAND>

Arguments:
  <URL>
  <COMMAND>

Options:
  -i, --interval <INTERVAL>  [default: 3]
  -h, --help                 Print help
  -V, --version              Print version
```

### Example

```
checkup https://shopping.com/item/cool_thing_you_want flash_lights.sh
```

### Demo

![checkup Demo](https://raw.githubusercontent.com/gusruben/checkup/refs/heads/main/checkup.gif)