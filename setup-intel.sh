#!/usr/bin/env sh

grubby --update-kernel=ALL --remove-args="rd.driver.blacklist=nouveau modprobe.blacklist=nouveau nvidia-drm.modeset=1"
grubby --update-kernel=ALL --args="rd.driver.blacklist=nvidia,nvidia_uvm,nvidia_drm,nvidia_modeset modprobe.blacklist=nvidia,nvidia_uvm,nvidia_drm,nvidia_modeset nouveau.modeset=0"
