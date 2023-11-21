# Whisper Keys Engine
**This is only the engine behind [Whisper Keys](https://github.com/NotAJocke/whisper-keys).**

Although you can use this as a CLI version, if you want/need a super lightweight version.

## What is this project?
It's a little software that emulates the sound of a mechanical keyboard when you type. It's a rewrite of [Mechvibes](https://github.com/hainguyents13/mechvibes) in rust to be more lightweight and faster.

## Features
Add your soundpacks, you can choose between them at startup and at runtime. There's also a translater between mechvibes config and whisper-keys config, trying to be as easy as possible to switch.

## Download
Paste this command in your terminal to download and install whisper-keys-engine.
```bash
curl -sSL https://raw.githubusercontent.com/NotAJocke/whisper-keys-engine/main/install.sh | bash
```
If you have any problem with the script, please open an issue.

If your OS/arch isn't supported, you can build it yourself.
### Build
Requirement: 
- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Protobuf compiler (protoc)](https://github.com/protocolbuffers/protobuf/releases)
```bash
$ git clone https://github.com/NotAJocke/whisper-keys-engine.git
$ cd whisper-keys-engine
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

