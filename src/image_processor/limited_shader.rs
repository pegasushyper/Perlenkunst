use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_channel() {
        assert_eq!(proxy_channel(0, 6), 0);
    }
    #[test]
    fn white_channel() {
        assert_eq!(proxy_channel(u8::MAX, 6), u8::MAX);
    }
}

fn proxy_channel(channel: u8, color_gradients: u8) -> u8 {
    let get_saturation = |grade| unsafe {
        f64::to_int_unchecked::<u8>(
            (255.0 / f64::from(color_gradients)) * f64::from(grade))
    };

    let closest_grade = (0..=color_gradients)
        .min_by_key(|grade| channel_distance(channel, get_saturation(*grade))).unwrap();

    get_saturation(closest_grade)
}

fn closest_proxy(pixel: &Rgba<u8>, color_gradients: u8) -> Rgba<u8> {
    if pixel[3] <= ALPHA_THRESHOLD {
        return Rgba::<u8>([0; 4])
    }

    let best = |i| proxy_channel(pixel[i], color_gradients);

    Rgba::<u8>([best(0), best(1), best(2), u8::MAX])
}

pub fn render_proxy_image(original_image: &DynamicImage, color_gradients: u8) -> DynamicImage {
    let rgba_image = original_image.to_rgba8();
    let mut proxy_image = RgbaImage::new(original_image.width(), original_image.height());

    for x in 0..original_image.width() {
        for y in 0..original_image.height() {
            let pixel = rgba_image.get_pixel(x, y);

            proxy_image.put_pixel(x, y, closest_proxy(pixel, color_gradients));
        }
    }

    DynamicImage::ImageRgba8(proxy_image)
}
