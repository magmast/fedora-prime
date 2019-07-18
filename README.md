# Fedora prime
> Switching between intel/nvidia drivers made simple.

Fedora prime is a program that allows you to switch between intel/nvidia drivers
in simple way. When you're switching to intel, it blacklists all nvidia modules
and sets nouveau options to run intel gpu and disable nvidia gpu. Whe you switch
to nvidia, it does the opposite.

The only thing that it touches is `/etc/modprobe.d/fedora-prime.conf` and
`/var/lib/fedora-prime/mode`. In the first file is done all the switching and
the second one contains info about current mode.

## Getting started

### Prerequesites

- [RPM Fusion nvidia drivers](https://rpmfusion.org/Howto/NVIDIA) or
  [negativo17 nvidia drivers](https://negativo17.org/nvidia-driver)

### Installation

I'm experimenting with copr repos, but I think I'm to stupid to make it work :(
If you want to help me check [this issue](https://github.com/magmast/fedora-prime/issues/1).
For now you can compile it from source or get `.rpm` package from
[releases](https://github.com/magmast/fedora-prime/releases).

### Usage

To use intel run:

```sh
sudo fedora-prime intel
```

To use nvidia run:

```sh
sudo fedora-prime nvidia
```

After switching you must reboot.

To check if it's working run:

```sh
glxinfo | grep 'OpenGL renderer'
```

### Uninstallation

Just remove the binary from `/usr/bin` and `/etc/modprobe.d/fedora-prime.conf`:

```sh
sudo rm /etc/modprobe.d/fedora-prime.conf /usr/bin/fedora-prime
```

### Building from source

Cd into directory containing project and run:

```sh
cargo build
```

## License

This project is licensed under MIT License. See [LICENSE](LICENSE) file for
details.
