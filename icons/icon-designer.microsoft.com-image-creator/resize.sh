#!/bin/bash

mkdir resized
convert iced.png -resize 512x512 resized/512x512.png
convert iced.png -resize 128x128 resized/128x128.png
convert iced.png -resize 64x64 resized/64x64.png
convert iced.png -resize 32x32 resized/32x32.png
