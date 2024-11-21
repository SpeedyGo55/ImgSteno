// Image Steganography
use std::path::Path;
use image::{ImageReader, RgbaImage};
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.contains(&"--help".to_string()) {
        display_help();
        return;
    }
    if args[1] == "--encrypt" && args.len() == 4 {
        let path = Path::new(&args[2]);
        let text = &args[3];

        let img = open_img(path);
        let img = embed_in_img(&img, text);
        img.save("output.png").unwrap();
    } else if args[1] == "--decrypt" && args.len() == 3 && args[2].ends_with(".png") {
        let path = Path::new(&args[2]);
        let img = open_img(path);
        let text = extract_text(&img);
        println!("{}", text);
    } else {
        display_help();
    }
}

fn display_help() {
    println!("Usage: ImgSteno [OPTION] [IMAGE_PATH] [TEXT]");
    println!("Options:");
    println!("  --encrypt [IMAGE_PATH] [TEXT] : Encrypt text in image. output.png will be created. Dont convert output.png to jpg or any other format");
    println!("  --decrypt [IMAGE_PATH] : Decrypt text from image. Image must be a PNG file");
}

fn open_img(path: &Path) -> RgbaImage{
    let img = ImageReader::open(path).unwrap().with_guessed_format().unwrap().decode().unwrap();
    img.to_rgba8()
}

fn img_to_bin(img: &RgbaImage) -> Vec<u8> {
    let mut bin = Vec::new();
    for pixel in img.pixels() {
        for subpixel in pixel.0 {
            bin.push(subpixel.to_ne_bytes()[0]);
        }
    }
    bin
}
fn embed_in_img(img: &RgbaImage, text: &str) -> RgbaImage {
    let bin = img_to_bin(img);
    let width = img.width();
    let height = img.height();
    let text = text.to_string() + "\0";
    let text = text.as_bytes();
    let mut text_vec = Vec::new();
    for byte in text {
        for i in (0..8).rev() {
            text_vec.push(get_bit(byte, i));
        }
    }
    let mut res = Vec::new();
    for (subpixel, index) in bin.iter().zip((0..bin.len()*2).step_by(2)) {
        // manipulate subpixel
        if index >= text_vec.len() {
            res.push(*subpixel);
            continue;
        }
        let mut temp = set_bit(subpixel, 0, text_vec[index]);
        temp = set_bit(&temp, 1, text_vec[index+1]);
        res.push(temp);
    }
    let img = RgbaImage::from_vec(width, height, res).unwrap();
    img
}

fn set_bit(x: &u8, idx: u8, b: bool) -> u8 {
    let mask = !(1 << idx);
    let flag = (b as u8) << idx;
    x & mask | flag
}

fn get_bit(x: &u8, idx: u8) -> bool {
    (x & (1 << idx)) != 0
}

fn extract_text(img: &RgbaImage) -> String {
    let bin = img_to_bin(img);
    let mut res = Vec::new();
    for subpixel in bin.iter() {
        let bit1 = get_bit(subpixel, 0);
        let bit2 = get_bit(subpixel, 1);
        res.push(bit1);
        res.push(bit2);
    }
    let mut text = String::new();
    for i in (0..res.len()).step_by(8) {
        let mut byte = 0;
        for j in 0..8 {
            byte = byte << 1;
            byte += res[i+j] as u8;
        }
        text.push(byte as char);
    }
    text = text.split("\0").collect::<Vec<_>>()[0].to_string();
    text
}