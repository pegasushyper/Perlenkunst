use image::{DynamicImage, RgbaImage, Rgba};

const CONTRAST_THRESHOLD: u32 = 50;
const ALPHA_THRESHOLD: u8 = 30;

pub mod contrast_shader;

pub mod limited_shader; 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_hex() {
        assert_eq!(util::hex_to_rgba("ffffff"), Rgba::<u8>([u8::MAX; 4]));
    }
    #[test]
    fn black_hex() {
        assert_eq!(util::hex_to_rgba("000000"), Rgba::<u8>([0, 0, 0, u8::MAX]));
    }
    #[test]
    #[should_panic]
    fn incorrect_hex_length() {
        let _ = util::hex_to_rgba("1234567");
    }
    
    #[test]
    fn contrast_group_update() {
        let white: contrast_shader::ContrastGroup = contrast_shader::ContrastGroup::init(&Rgba::<u8>([u8::MAX; 4]));

        assert_eq!(white.average_rgba(), Rgba::<u8>([255; 4]));
    }

    #[test]
    fn distance_0() {
        let white = Rgba::<u8>([255; 4]);
        assert_eq!(color_distance(&white, &white), 0);
    }
    #[test]
    fn distance_60() {
        let white = Rgba::<u8>([255; 4]);
        let grey = Rgba::<u8>([235; 4]);
        assert_eq!(color_distance(&white, &grey), 60);
    }

    fn existance(list: &Vec<contrast_shader::ContrastGroup>) -> Result<(), ()> {
        let white = Rgba::<u8>([255; 4]);
        if let Some(_) = contrast_shader::check_for_existance(&white, &list) {
            return Ok(());
        } else {
            return Err(());
        }
    }
    #[test]
    #[ignore]
    fn exists() {
        let white_list = vec![contrast_shader::ContrastGroup::init(&Rgba::<u8>([255; 4]))];
        
        let _ = existance(&white_list).expect(format!("list containing white: {:?} ; rgba: {:?}", white_list, white_list[0].average_rgba()).as_str());
    }
    #[test]
    #[ignore]
    #[should_panic]
    fn does_not_exist() {
        let black_list = vec![contrast_shader::ContrastGroup::init(&Rgba::<u8>([0; 4]))];
        
        let _ = existance(&black_list).expect(format!("list containing black: {:?} ; rgba: {:?}", black_list, black_list[0].average_rgba()).as_str());
    }
}

pub mod io {
    use image::io::Reader;
    use image::DynamicImage;

    pub fn load_image(path: &str) -> DynamicImage {
        // TODO proper error handling
        Reader::open(path)
            .expect("couldn't open image")
            .decode()
            .expect("couldn't decode image properly (maybe faulty encoding)")
    }
}

mod util {
    use hex_string::HexString;
    use image::Rgba;

    // only full transparency supported
    fn adjust_to_alpha(_: Rgba<u8>) -> Rgba<u8> {
        unimplemented!();
    }
    
    // TODO outsource to config reader
    fn rgb_vec_to_rgba(rgb_vec: Vec<u8>) -> Rgba<u8> {
        Rgba::<u8>([rgb_vec[0], rgb_vec[1], rgb_vec[2], u8::MAX])
    }
    pub fn hex_to_rgba(rgb_hex: &str) -> Rgba<u8> {
        if rgb_hex.len() != 6 {
            panic!("incorrect length of rgb hex: {} should be 6", rgb_hex.len());
        }

        // TODO proper error handling
        let rgb_vec = HexString::from_string(rgb_hex.to_lowercase().as_str())
            .unwrap()
            .as_bytes();

        rgb_vec_to_rgba(rgb_vec)
    }
}

pub fn fit_in_canvas(original: &DynamicImage, limit: u32) -> DynamicImage {
    original.resize(limit, limit, image::imageops::FilterType::CatmullRom)
}

fn channel_distance(channel1: u8, channel2: u8) -> u8 {
    u8::try_from(
        (i16::from(channel1) - i16::from(channel2))
        .abs())
        .expect("should always convert fine")
}

fn color_distance(pixel1: &Rgba<u8>, pixel2: &Rgba<u8>) -> u32 {
    let mut score: u32 = 0;

    for i in 0..3 {
        score += u32::from(channel_distance(pixel1[i], pixel2[i]));
    }

    score
}
