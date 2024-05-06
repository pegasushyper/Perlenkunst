//use std::collections::HashMap;
//use std::fs::File;
use image::DynamicImage;

mod image_processor;
mod cli;

fn main() {
    let args = cli::cli();

    let img = image_processor::io::load_image(args.image_path);

    let img_resize = image_processor::fit_in_canvas(&img, args.dimension);

    let img_canvas: DynamicImage;
    match args.shader {
        cli::Shader::Graded(n) => {
            img_canvas = image_processor::limited_shader::render_proxy_image(&img_resize, n);
        }
        cli::Shader::Contrast => {
            let overlay: image_processor::contrast_shader::Overlay;
            let colorspace: Vec<image_processor::contrast_shader::Color>;
            (overlay, colorspace) = image_processor::contrast_shader::line_overlay(&img_resize);

            img_canvas = image_processor::contrast_shader::render(overlay, colorspace);
        }
        cli::Shader::Palette(_c) => { unimplemented!() },
    }

    let _ = img_canvas.save(args.out_path);
}
