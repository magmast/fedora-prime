# Fedora prime

Simple program written in rust which manages xorg and modprobe config to switch
between igpu and dgpu. It does that by adding or removing xorg config
(/etc/X11/xorg.conf.d/10-fedora-prime.conf) and modprobe file
(/etc/modprobe.d/fedora-prime.conf).

This script is so simple that it should work on any distro, but if it isn't, 
then it's so short and simple that it can be ported in 10 minutes.

## Prerequesites

- [bbswitch](https://github.com/Bumblebee-Project/bbswitch) (to disable dgpu)
- [nvidia drivers](https://rpmfusion.org/Howto/NVIDIA)
- [rustup](https://rustup.rs/) (only to install with cargo)

## Installation

First you need to install rust toolchain:

```
rustup default stable
```

Now you can install the program:

```
cargo install fedora-prime
```

Make sure to have `$HOME/.cargo/bin` in your `$PATH`.

## Usage

To change between cards run:

```sh
sudo fedora-prime switch [intel/nvidia]
```

If you switch to intel, disable dgpu with:

```sh
sudo fedora-prime disable-gpu
```

This step will be done automatically, but I want to do that without complicating
code to much (for example adding systemd service), so I need to think how to do
it.

## Configuration

Configuration file is `/etc/fedora-prime.toml`. Check 
[config.example.toml](config.example.toml) to see example configuration.

## License

This project is licensed under MIT License. See [LICENSE](LICENSE) file for
details.
