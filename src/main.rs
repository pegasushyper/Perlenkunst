//use std::collections::HashMap;
//use std::fs::File;

mod image_processor;

const CANVAS_CONSTRAINTS: u32 = 480;
const GRADING: u8 = 4;      // this cubed is the number of possible colors; e.g. 4³ = 64 colors

fn main() {
    let project_root = String::from("/home/pegasushyper/programming/perlenkunst/");
    let img_name = "purepng.com-ford-mustang-red-carcarvehicletransportford-961524641401fbblv.png";
    let output_folder = project_root.clone() + "out/";

    print!("Loading Image into RAM... ");
    let img = image_processor::io::load_image((project_root.clone() + img_name).as_str());
    println!("done");

    print!("resizing... ");
    let img_resize = image_processor::fit_in_canvas(&img, CANVAS_CONSTRAINTS);
    println!("done");

    print!("Render Image with Limited Colorspace... ");
    let img_limited = image_processor::limited_shader::render_proxy_image(&img_resize, GRADING);
    println!("done");

    print!("Generate Contrast Overlay for Shader...");
    let (overlay, groups) = image_processor::contrast_shader::generate_contrast_overlay(&img_resize);
    println!("done");

    print!("Render Image with Contrast Shader... ");
    let img_contrast = image_processor::contrast_shader::render_contrast_image(overlay, groups);
    println!("done");

    print!("Saving Results... ");
    let _ = img_resize.save(output_folder.clone() + "scaled.png");
    let _ = img_limited.save(output_folder.clone() + "canvas.png");
    let _ = img_contrast.save(output_folder.clone() + "canvas2.png");
    println!("done");
}
