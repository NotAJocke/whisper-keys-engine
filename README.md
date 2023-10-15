# Whisper Keys
Little side project that I made for myself and few friends, rewrite of the initial project [Mechvibes](https://github.com/hainguyents13/mechvibes) (#rewriteInRust). The goal was to pratice rust and make a more efficient version of the initial project.

# What is this ?
As I previously said it's a rewrite of Mechvibes, a little software that emulates the sound of a mechanical keyword when you type, your coworkers and wallet will thank you.

# State
Windows isn't officially supported, but it should work, if you have any problem, please open an issue.

At the moment there's only a CLI version, I'm planning to make a GUI version, but there's a problem with the lib that I'm using to listen to keyboard events, so I'm waiting for a fix.

This is only a rewrite of mechvibes, and not __yet__ of mechvibes++ (though I'm planning to do it).

# Features
Add your soundpacks, you can choose between them at startup. There's also a translater between mechvibes config and whisper-keys config, trying to be as easy as possible to switch.

# Installation
### Binaries for MacOS and Linux
```bash
curl -sSL https://raw.githubusercontent.com/NotAJocke/whisper-keys/main/install.sh | bash
```
If you have any problem with the script, please open an issue.

Atm, there's no binaries for Windows and MacOS x86_64, but you can build it yourself.

### Build
Requirement: [Rust toolchain](https://www.rust-lang.org/tools/install)
```bash
$ git clone https://github.com/NotAJocke/whisper-keys.git
$ cd whisper-keys
$ cargo install --path .
```

# Usage
Run once to generate the pack folder, then add your soundpacks to the folder (`~/WhisperKeys`), and run again to choose your soundpack.
```bash
$ whisper-keys
```

### Translating mechvibes config
```bash
$ whisper-keys --translate <path to mechvibes soundpack folder>
```

### Generating a new empty soundpack
This will create a folder `Pack_Name` in the location where you run the command.
```bash
$ whisper-keys --generate
```

# Getting pre-made packs
Join the [mechvibes Discord](https://discord.com/invite/MMVrhWxa4w) and look for the channel `#custom-soundpacks`. These soundpacks can be translated to Whisper Keys easily.

Join the [WhisperKeys's Host Discord](https://discord.gg/NBrkFgWnc2)

# Migrating from v1
To migrate from v1, open your config files and edit accordingly:

*old config*
```json
{
  "space": "spacebar.wav",
  ...
}
```

*new config*
```json
{
  "creator": "",
  "source": "",
  "keys_default_volume": "50", # this is in percent
  "keys": {
    "space": "spacebar",
    ...
  }
}
```