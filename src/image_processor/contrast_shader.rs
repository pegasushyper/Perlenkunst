use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transparency_true() {
        let rgba = Rgba::<u8>([0; 4]);

        assert!(counts_transparent(&rgba));
    }
    #[test]
    fn transparency_false() {
        let rgba = Rgba::<u8>([u8::MAX; 4]);

        assert!(!counts_transparent(&rgba));
    }

    #[test]
    #[should_panic]
    fn empty_color() {
        let empty = Color::new();

        let _ = empty.rgba().unwrap();
    }

    #[test]
    fn grey_average() {
        let black = Rgba::<u8>([0, 0, 0, u8::MAX]);
        let white = Rgba::<u8>([u8::MAX; 4]);
        
        let mut mix = Color::new();
        mix.add(&black);
        mix.add(&white);
        
        let grey = Rgba::<u8>([127, 127, 127, u8::MAX]);
        
        assert_eq!(mix.rgba().unwrap(), grey);
    }

    fn white_existance(colorspace: &Vec<Color>) -> Result<(), ()> {
        let white = Rgba::<u8>([u8::MAX; 4]);

        match colorspace_index(&white, colorspace) {
            Some(_) => Ok(()),
            None => Err(()),
        }
    }
    #[test]
    fn exists() {
        let white = Rgba::<u8>([u8::MAX; 4]);
        let mut white_list = vec![Color::new()];
        white_list[0].add(&white);

        let _ = white_existance(&white_list).expect("");
    }
    #[test]
    #[should_panic]
    fn does_not_exist() {
        let black = Rgba::<u8>([0; 4]);
        let mut black_list = vec![Color::new()];
        black_list[0].add(&black);

        let _ = white_existance(&black_list).expect("");
    }
}

pub type Overlay = Vec<Vec<Option<usize>>>;

pub struct Color {
    rgb: [u32; 3],
    amount: u32,
}

impl Color {
    pub fn new() -> Self {
        Self {
            rgb: [0, 0, 0],
            amount: 0,
        }
    }
    pub fn add(&mut self, rgba: &Rgba<u8>) {
        self.rgb[0] += u32::from(rgba[0]);
        self.rgb[1] += u32::from(rgba[1]);
        self.rgb[2] += u32::from(rgba[2]);
        self.amount += 1;
    }
    pub fn rgba(&self) -> Option<Rgba<u8>> {
        if self.amount == 0 {
            return None;
        }

        let channel_avg = |i: usize| (self.rgb[i] / self.amount).try_into().unwrap();

        Some(Rgba::<u8>([channel_avg(0), channel_avg(1), channel_avg(2), u8::MAX]))
    }
}

fn counts_transparent(rgba: &Rgba<u8>) -> bool {
    if rgba[3] < ALPHA_THRESHOLD {
        return true;
    }
    false
}

fn colorspace_index(rgba: &Rgba<u8>, colorspace: &Vec<Color>, contrast_threshold: u32) -> Option<usize> {
    if colorspace.len() == 0 {
        eprintln!("Empty Colorspace, probably first Color");
        return None;
    }    

    for i in 0..colorspace.len() {
        let average = match colorspace[i].rgba() {
            Some(avg) => avg,
            None => {
                eprintln!("Color at index {} empty, probably first Color", i);
                return None;
            }
        };

        if color_distance(rgba, &average) < contrast_threshold {
            return Some(i);
        }
    }
    None
}

pub fn line_overlay(source_image: &DynamicImage, contrast_threshold: u32) -> (Overlay, Vec<Color>) {
    let width: usize = source_image.width().try_into().unwrap();
    let height: usize = source_image.height().try_into().unwrap();
    let source = source_image.to_rgba8();

    let mut colorspace: Vec<Color> = Vec::new();
    let mut overlay: Overlay = vec![vec![None; height]; width];

    for pixel in source.enumerate_pixels() {
        let x: usize = pixel.0.try_into().unwrap();
        let y: usize = pixel.1.try_into().unwrap();
        let rgba = pixel.2;

        if counts_transparent(rgba) {
            continue
        }

        match colorspace_index(rgba, &colorspace, contrast_threshold) {
            Some(i) => overlay[x][y] = Some(i),
            None => {
                let mut new_color = Color::new();
                new_color.add(rgba);
                colorspace.push(new_color);
                overlay[x][y] = Some(colorspace.len()-1);
            }
        }
    }

    (overlay, colorspace)
}

pub fn render(overlay: Overlay, colorspace: Vec<Color>) -> DynamicImage {
    let width = overlay.len();
    let height = overlay[0].len();

    let mut image = DynamicImage::new_rgba8(
        width.try_into().unwrap(),
        height.try_into().unwrap()
        );
    let rgba_image = image.as_mut_rgba8().unwrap();

    for x in 0..width {
        for y in 0..height {
            // TODO Error handling
            let rgba = match overlay[x][y] {
                Some(i) => colorspace[i].rgba().unwrap(),
                None => Rgba::<u8>([0; 4]),
                };

            rgba_image.put_pixel(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                rgba
                );
        }
    }
    image
}
