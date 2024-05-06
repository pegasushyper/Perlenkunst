# Perlenkunst

Perlenkunst is an WIP open-source image interpreter for pixel art generation and bead art inspiration written in rust.

## Usage

Download the source over the browser or by running

```BASH
$ git clone https://github.com/pegasushyper/perlenkunst
```

Then build the project by running

```BASH
$ cargo build --release
```

The binary, called `perlenkunst[.exe]`, will be located in `./target/release/`

## Shaders

### Limited Colorspace Shader

Each color channel will be split into n+1 amount of gradiants. The total number of colors will be n³.

For each pixel the closest relative on this colorspace will be selected.

### Contrast Grouping Shader

Image will be read vertically line-by-line. Pixels get grouped by contrast (added distance between the colors). The groups average color values will be used in the output image.

### Palette

WIP

## Copyright Notice

I do not claim ownership over the character Elsa (Frozen). It rightfully belongs to Walt Disney Studios.
