# ImgSteno
ImgSteno is a program written in Rust that can hide text in Images and get it back out.

## Installation
Download the latest release from the [releases page](https://github.com/SpeedyGo55/ImgSteno/releases/latest)

## Usage
To hide text in an image:
```bash
./ImgSteno.exe --encrypt <image_path> "<text>"
```

To get text out of an image (image must be a png):
```bash
./ImgSteno --decrypt <image_path>
```