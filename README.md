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

The binary will be located in `./target/release/`

## Shaders

### Limited Colorspace Shader

Each color channel will be split into n+1 amount of gradiants. The total number of colors will be n³.

For each pixel the closest relative on this colorspace will be selected.

### Contrast Grouping Shader

#### Line mode

Image will be read vertically line-by-line. Each pixel is compared to the previous one. If they're inside of the threshold they will get grouped. If not, first it's checked if there is already a group that the pixel would fit in. If there is not, then a new group gets created. At the end groups will be assigned an average color value, which is then projected onto every pixel in the group on the final image.

#### Crawler mode

WIP

### Palette

WIP

## Copyright Notice

I do not claim ownership over the character Elsa (Frozen). It rightfully belongs to Walt Disney Studios.
