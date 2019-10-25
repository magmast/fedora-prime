# Fedora prime
> Switching between intel/nvidia drivers made simple.

This repository contains just two scripts `setup-intel.sh` and `setup-nvidia.sh`.
Those scripts are modifying kernel parameters to enable/disable nouveau and nvidia
drivers. Doing so should switch which gpu is used.

While nouveau driver is loaded, it should disable discrete gpu. But there is a
chance that it will not. Unfortunately it doesn't on my laptop, so I decided
to change my linux distro. This means that I'll no longer actively maintain
fedora-prime, but I'll still accept pull requests or maybe even work on some
issues. But still, I encourage you to try fedora-prime. If it will not work for
you, try to use `0.1.1` version (see releases or tags).

## Getting started

### Prerequesites

- [RPM Fusion nvidia drivers](https://rpmfusion.org/Howto/NVIDIA) or
  [negativo17 nvidia drivers](https://negativo17.org/nvidia-driver)

### Installation

Copy and paste `setup-intel.sh` and `setup-nvidia.sh` into some directory on
your PC (for example `/usr/bin` or `~/bin`).

### Usage

To use intel run:

```sh
sudo ./setup-intel.sh
```

To use nvidia run:

```sh
sudo ./setup-nvidia.sh
```

Now you must reboot.

To check if it's working run:

```sh
glxinfo | grep 'OpenGL renderer'
```

This will print which gpu is used. But even if it's intel, nvidia gpu may be
still on. To check that use powertop.

## License

This project is licensed under MIT License. See [LICENSE](LICENSE) file for
details.
