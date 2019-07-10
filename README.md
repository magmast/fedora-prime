# Fedora prime

Simple program written in rust which manages xorg and modprobe config to switch
between igpu and dgpu. It does that by adding or removing xorg config
(/etc/X11/xorg.conf.d/10-fedora-prime.conf) and modprobe file
(/etc/modprobe.d/fedora-prime.conf).

This script is so simple that it should work on any distro, but if it isn't, 
then it's so short and simple that it can be ported in 10 minutes.

## Prerequesites

- [bbswitch](https://github.com/Bumblebee-Project/bbswitch)
- [nvidia drivers](https://rpmfusion.org/Howto/NVIDIA)
- [rustup](https://rustup.rs/) (only to install with cargo)

## Installation

```
cargo install fedora-prime
```

## Usage

To change between cards run:

```sh
fedora-prime [intel/nvidia]
```

If you switch to intel, disable dgpu with:

```sh
sudo modprobe bbswitch
sudo tee /proc/acpi/bbswitch <<<OFF
```

This step will be done automatically, but I want to do that without complicating
code to much (for example adding systemd service), so I need to think how to do
it.

## License

This project is licensed under MIT License. See [LICENSE](LICENSE) file for
details.
