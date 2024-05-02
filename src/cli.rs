use std::path::PathBuf;
use clap::{arg, value_parser, Command};

pub enum Shader {
    Graded(u8),
    Contrast(String),
    Palette(PathBuf),
}

pub struct Arguments {
    pub image_path: PathBuf,
    pub out_path: PathBuf,
    pub dimension: u32,
    pub shader: Shader,
}

pub fn cli() -> Arguments {
    let matches = Command::new("perlenkunst")
        .version("0.1")
        .about("Create pixel art")
        .arg_required_else_help(true)
        .arg(arg!(-d --dimension <VALUE> "maximum edge length in pixels")
            .value_parser(value_parser!(u32))
            .default_value("360"),
        )
        .arg(arg!(-o --out <FILE> "custom output path/filename")
            .value_parser(value_parser!(PathBuf))
            .default_value("canvas.png"),
        )
        .subcommand(Command::new("graded")
            .about("devides every channel into n+1 color gradiations")
            .arg(arg!(-n --gradiants <VALUE>)
                .value_parser(value_parser!(u8))
                .default_value("3"),
            ),
        )
        .subcommand(Command::new("contrast")
            .about("groups low contrast pixels")
            .arg(arg!(-m --method <NAME> "crawler/line")
                .default_value("line"),
            ),
        )
        .subcommand(Command::new("palette")
            .about("match pixels to colors on the palette")
            .arg(arg!(-c --config <FILE> "path to palette")
                .value_parser(value_parser!(PathBuf))
            ),
        )
        .arg(arg!([IMAGE] "path to image")
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let mut args = Arguments {
        image_path: PathBuf::new(),
        out_path: PathBuf::new(),
        dimension: 360,
        shader: Shader::Graded(3),
    };

    if let Some(dimension) = matches.get_one::<u32>("dimension") {
        args.dimension = *dimension;
    }

    if let Some(out_path) = matches.get_one::<PathBuf>("out") {
        args.out_path = out_path.to_path_buf();
    }
    
    match matches.subcommand() {
        Some(("graded", sub_matches)) => {
            if let Some(n) = sub_matches.get_one::<u8>("gradiants") {
                args.shader = Shader::Graded(*n);
            }
        }
        Some(("contrast", sub_matches)) => {
            if let Some(m) = sub_matches.get_one::<String>("method") {
                args.shader = Shader::Contrast(m.to_string());
            }
        }
        Some(("palette", sub_matches)) => {
            if let Some(c) = sub_matches.get_one::<PathBuf>("config") {
                args.shader = Shader::Palette(c.to_path_buf());
            }
        }
        _ => panic!("There is no command like that"),
    }
    
    if let Some(image_path) = matches.get_one::<PathBuf>("IMAGE") {
        args.image_path = image_path.to_path_buf();
    }

    args
}
