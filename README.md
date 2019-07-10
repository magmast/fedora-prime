# Fedora prime

Simple program written in rust which manages xorg and modprobe config to switch
between igpu and dgpu. It does that by adding or removing xorg config
(/etc/X11/xorg.conf.d/10-fedora-prime.conf) and modprobe file
(/etc/modprobe.d/fedora-prime.conf).

## Prerequesites

If you don't want to disable dgpu when not using it, there are no prerequesites.
But you propably want and in this case you need to have bbswitch installed.

## Installation

```
cargo install fedora-prime
```

## Usage

```sh
fedora-prime [intel/nvidia]
```

If you switch to intel, disable dgpu with:

```sh
sudo modprobe bbswitch
sudo tee /proc/acpi/bbswitch <<<OFF
```

## License

This project is licensed under MIT License. See [LICENSE](LICENSE) file for
details.
