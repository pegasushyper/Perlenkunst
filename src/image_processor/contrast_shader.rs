use super::*;

pub type ContrastOverlay = Vec<Vec<usize>>;
#[derive(Debug)]
pub struct ContrastGroup {
    total_red: u32,
    total_green: u32,
    total_blue: u32,
    amount: u32,
    transparent: bool,
}

impl ContrastGroup {
    pub fn new() -> Self {
        Self {
            total_red: 0,
            total_green: 0,
            total_blue: 0,
            amount: 0,
            transparent: false,
        }
    }

    pub fn init(pixel: &Rgba<u8>) -> Self {
        Self {
            total_red: u32::from(pixel[0]),
            total_green: u32::from(pixel[1]),
            total_blue: u32::from(pixel[2]),
            amount: 1,
            transparent: false,
        }
    }

    pub fn new_transparent() -> Self {
        Self {
            total_red: 0,
            total_green: 0,
            total_blue: 0,
            amount: 0,
            transparent: true,
        }
    }

    pub fn average_rgba(&self) -> Rgba<u8> {
        // TODO should not be needed
        if self.amount == 0 {
            return Rgba::<u8>([0, 0, 0, 255]);
        }

        if self.transparent {
            return Rgba::<u8>([0; 4]);
        }

        let avg_red: u8 = (self.total_red / self.amount).try_into().unwrap();
        let avg_green: u8 = (self.total_green / self.amount).try_into().unwrap();
        let avg_blue: u8 = (self.total_blue / self.amount).try_into().unwrap();

        Rgba::<u8>([avg_red, avg_green, avg_blue, u8::MAX])
    }

    pub fn with_new_instance(&self, pixel: &Rgba<u8>) -> Self {
        if self.transparent {
            return Self {
                amount: self.amount + 1,
                total_red: 0,
                total_green: 0,
                total_blue: 0,
                transparent: true,
            };
        }

        Self {
            amount: self.amount + 1,
            total_red: self.total_red + u32::from(pixel[0]),
            total_green:  self.total_green + u32::from(pixel[1]),
            total_blue: self.total_blue + u32::from(pixel[2]),
            transparent: false,
        }
    }
}

pub fn check_for_existance(pixel: &Rgba<u8>, groups: &Vec<ContrastGroup>) -> Option<usize> {
    for i in 1..groups.len() {
        if color_distance(pixel, &groups[i].average_rgba()) < CONTRAST_THRESHOLD {
            return Some(i);
        }
    }

    None
}

pub fn generate_contrast_overlay(source_image: &DynamicImage) -> (ContrastOverlay, Vec<ContrastGroup>) {
    let width: usize = source_image.width().try_into().unwrap();
    let height: usize = source_image.height().try_into().unwrap();

    let source_image: RgbaImage = source_image.to_rgba8();

    let mut color_groups: Vec<ContrastGroup> = vec![ContrastGroup::new_transparent(), ContrastGroup::new()];    //[0] reserved for transparency
    let mut overlay: ContrastOverlay = vec![vec![0usize; height]; width];

    let mut id: usize = 1;
    // TODO compare to results when only using average
    let mut prev_pixel: &Rgba<u8> = source_image.get_pixel(0, 0);

    for x in 0..width {
        for y in 0..height {
            let pixel = source_image.get_pixel(
                x.try_into().unwrap(),
                y.try_into().unwrap());

            if pixel[3] < ALPHA_THRESHOLD {
                overlay[x][y] = 0;
                color_groups[0] = color_groups[0].with_new_instance(pixel);
                prev_pixel = pixel;
                continue;
            }
            
            let contrast_score = color_distance(prev_pixel, pixel);

            if contrast_score > CONTRAST_THRESHOLD {
                if let Some(index) = check_for_existance(pixel, &color_groups) {
                    id = index;
                } else {
                    id = color_groups.len() - 1;
                    color_groups.push(ContrastGroup::new());
                }
            }

            overlay[x][y] = id;

            color_groups[id] = color_groups[id].with_new_instance(pixel);

            prev_pixel = pixel;
        }
    }
    //println!("groups: {}", color_groups.len());

    (overlay, color_groups)
}

pub fn render_contrast_image(overlay: ContrastOverlay, contrasts: Vec<ContrastGroup>) -> DynamicImage {
    let width = overlay.len();
    let height = overlay[0].len();

    let mut contrast_image = DynamicImage::new_rgba8(
        width.try_into().unwrap(),
        height.try_into().unwrap());
    let contrast_rgba = &mut contrast_image.as_mut_rgba8().unwrap();

    for x in 0..width {
        for y in 0..height {
            let contrast_pixel = &contrasts[overlay[x][y]];
            contrast_rgba.put_pixel(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                contrast_pixel.average_rgba());
        }
    }

    contrast_image
}
