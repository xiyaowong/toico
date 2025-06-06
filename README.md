# toico

toico is a tool for converting image files to ICO format

# Installation

Download the executable from the releases page, or install via cargo:

```bash
cargo install --git https://github.com/xiyaowong/toico
```

# Usage

```bash
toico -h
```

```
Convert images to ICO format

Usage: toico [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  The image file to convert

Options:
  -s, --size <SIZE>      The size of the icon in pixels: 16, 24, 32, 48, 64, 128, 256 [default: 64]
  -o, --output <OUTPUT>  The output file name, defaults to the input file name

                         If the --all option is specified, files will be saved in a directory named <output>_ico
                         If --all is not specified, the file will be saved as <output>.ico

  -a, --all              Whether to generate all icon sizes (16, 24, 32, 48, 64, 128, 256)
  -f, --force            Force overwrite existing files
  -h, --help             Print help
  -V, --version          Print version


Examples:
    toico image.png --size 64
    toico image.png -s 64 --output my_icon

Tips:
    Drag and drop an image file onto the executable to convert it to ICO format.
```

# License

MIT