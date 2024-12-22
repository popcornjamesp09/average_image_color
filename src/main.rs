use image::GenericImageView;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Add the file path to the image you want to be averaged");
        println!("Usage: {} image.png", args[0]);
        std::process::exit(0)
    }

    let img_path: &String = &args[1];

    let img_path_extinsion: String = get_img_path_extinsion((&img_path).to_string());

    let img = image::open(img_path).unwrap();

    let (img_dimension_x, img_dimension_y) = img.dimensions();

    let color: [u8 ; 3] = get_average_color(img, img_dimension_x, img_dimension_y); 

    let new_image: image::ImageBuffer<image::Rgb<u8>, Vec<_>> = make_image_buf(img_dimension_x, img_dimension_y, color);

    new_image.save(&(img_path.to_string() + ".new" + &img_path_extinsion)).unwrap()
}

// This is needed because you have to have a valid image extension for img.save()
// Is it the best no, does it work yes
fn get_img_path_extinsion(mut img_path: String) -> String {
    let len = img_path.chars().count() - 4;
    let _ = img_path.drain(0..len).for_each(|_| {});
    return img_path;
}


fn get_average_color(img: image::DynamicImage, img_dimension_x: u32, img_dimension_y: u32) -> [u8 ; 3] {
    // Because IDK another way
    // This one works :)
    let mut r_value: u32 = 0;
    let mut g_value: u32 = 0;
    let mut b_value: u32 = 0;


    for x in 1..img_dimension_x {
        for y in 1..img_dimension_y {
            let rgb_value = img.get_pixel(x, y);
            r_value += u32::from(rgb_value[0]); 
            g_value += u32::from(rgb_value[1]);
            b_value += u32::from(rgb_value[2]);
        }
    }

    // Needed to find the average color of an image
    let total_pixel_count = img_dimension_x * img_dimension_y;
    
    return [(r_value/total_pixel_count) as u8, (g_value/total_pixel_count) as u8, (b_value/total_pixel_count) as u8];

}

// Makes a image buffer and fills it with a color
fn make_image_buf(img_dimension_x: u32, img_dimension_y: u32, color: [u8 ; 3]) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut new_image = image::RgbImage::new(img_dimension_x, img_dimension_y);

    for x in 1..img_dimension_x {
        for y in 1..img_dimension_y {
            new_image.put_pixel(x, y, image::Rgb(color));
        }
    }

    return new_image
}