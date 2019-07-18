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
- [rustup](https://rustup.rs/) (only to install with cargo)

### Installation

Fedora prime isn't yet available as rpm package (but it will), so you need to
build it from source. Don't worry it's very simple :)

First you'll need to install rustup:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install rust toolchain and set is as default:

```sh
rustup default stable
```

Add `$HOME/.cargo/bin` to your path.

```sh
echo 'export PATH="$PATH:$HOME/.cargo/bin"' >> ~/.bash_profile
```

Install fedora prime:

```sh
cargo install fedora-prime
```

### Usage

To use intel run:

```sh
fedora-prime intel
```

To use nvidia run:

```sh
fedora-prime nvidia
```

After switching you must reboot.

To check if it's working run:

```sh
glxinfo | grep 'OpenGL renderer'
```

## License

This project is licensed under MIT License. See [LICENSE](LICENSE) file for
details.
