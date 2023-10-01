# SysTem

A system command bar that aims to speed up my workflow.

At the moment the package doesn't aim at the "general user" and is more focused on my specific needs.

**! Probably not linux compatible at the moment !**

## Features

- Integrated calculator (Done with the [evalexpr](https://docs.rs/evalexpr/latest/evalexpr/) crate)
- Quick command execution (Windows specific)
- WSL quick start (Windows specific)
- Not so smart git ghp key manager (Not tested on Linux)

## Setup

The only setup needed is the config. Go in the home directory of your user and create a `.system` file it will contain the config as json:
```json
{
    "gitkeys": {
        "default": "default_key",
        "keys": {
            "default_key": "key_one",
            "secondary_key": "secondary_key"
        }
    },
    "terminal": {
        "new": [ "command", "and", "args", "to create a new terminal", "%c", "is replaced by the terminal name" ]
    }
}
```