//use std::collections::HashMap;
//use std::fs::File;
use image::DynamicImage;

mod image_processor;
mod cli;

fn main() {
    let args = cli::cli();

    let img = image_processor::io::load_image(args.image_path);

    let img_resize = image_processor::fit_in_canvas(&img, args.dimension);

    let mut img_canvas: DynamicImage;
    match args.shader {
        cli::Shader::Graded(n) => {
            img_canvas = image_processor::limited_shader::render_proxy_image(&img_resize, n);
        }
        cli::Shader::Contrast(m) => {
            let (overlay, groups) = image_processor::contrast_shader::generate_contrast_overlay(&img_resize);
            img_canvas = image_processor::contrast_shader::render_contrast_image(overlay, groups);
        }
        cli::Shader::Palette(c) => { unimplemented!() },
    }

    let _ = img_canvas.save(args.out_path);
}
