use super::*;

fn proxy_channel(channel: u8, color_gradients: u8) -> u8 {
    let mut best_score: u8 = u8::MAX;
    let mut best_saturation: u8 = 0;
    for grade in 0..=color_gradients {
        let saturation = unsafe {
            f64::to_int_unchecked::<u8>(
                (256.0f64 / f64::from(color_gradients)) * f64::from(grade)
                )
        };
        let current_score = channel_distance(channel, saturation);

        if current_score > best_score { break; }

        best_score = current_score;
        best_saturation = saturation;
    }

    best_saturation
}

fn closest_proxy(pixel: &Rgba<u8>, color_gradients: u8) -> Rgba<u8> {
    if pixel[3] <= ALPHA_THRESHOLD {
        return Rgba::<u8>([0; 4])
    }

    let best_r = proxy_channel(pixel[0], color_gradients);
    let best_g = proxy_channel(pixel[1], color_gradients);
    let best_b = proxy_channel(pixel[2], color_gradients);

    Rgba::<u8>([best_r, best_g, best_b, u8::MAX])
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
