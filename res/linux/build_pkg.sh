#!/bin/sh
mkdir build-dir
flatpak-builder --repo=repo build-dir res/linux/org.jms.PokeRNGTool.json
flatpak build-bundle repo pokerngtool.flatpak org.jms.PokeRNGTool
rm -rf .flatpak-builder
rm -rf build-dir
rm -rf repo
