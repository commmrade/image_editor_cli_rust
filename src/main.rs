use image::{io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};
use core::fmt::Error;
use std::mem::swap;
use std::time::SystemTime;
use image::Rgb;
use image::RgbaImage;

fn main() {
    let start = SystemTime::now();
    let mut img : DynamicImage = ImageReader::open("img.jpg").unwrap().decode().unwrap();
    
    //Doing smth with the image...
    scale_down(&mut img, 3);
    blur_image(&mut img, 15);

    //TODO: Add user interaction

    
    //println!("{} s {}", img.width(), img.height());
    img.into_rgb8().save("here.jpg").unwrap(); //Saving
    let end = SystemTime::now(); //Timer

    println!("Took {} seconds", end.duration_since(start).unwrap().as_secs());
    //img.save("here.jpg").unwrap();
}

fn flip_horizontally(img : &mut DynamicImage)
{
    for x in 0..(img.width() / 2)
    {
        for y in 0..img.height()
        {
            let pixel1 = img.get_pixel(x, y);
            let pixel2 = img.get_pixel(img.width() - x - 1, y); 

            img.put_pixel(x, y, pixel2);
            img.put_pixel(img.width() - x - 1, y, pixel1);
        }
    }
}

fn flip_vertically(img : &mut DynamicImage)
{
    for x in 0..img.width()
    {
        for y in 0..(img.height() / 2)
        {
            let pixel1 = img.get_pixel(x, y);
            let pixel2 = img.get_pixel(x, img.height() - y - 1);

            img.put_pixel(x, y, pixel2);
            img.put_pixel(x, img.height() - y - 1, pixel1);
        }
    }
}

fn scale_down(mut img: &mut DynamicImage, scale: u32) {
    let mut imgbuf : ImageBuffer<Rgba<u8>, Vec<_>> = ImageBuffer::new(img.width() / scale, img.height() / scale);
    
    for x in 0..imgbuf.width() {
        for y in 0..imgbuf.height() {
            let pixel = img.get_pixel(x * scale, y * scale);
            
            imgbuf.put_pixel(x, y, pixel);
        }
    }
    *img = DynamicImage::ImageRgba8(imgbuf);
}

fn rotate_image(mut img : &mut DynamicImage)
{
    let mut imgb : ImageBuffer<Rgba<u8>, Vec<_>> = ImageBuffer::new(img.height(), img.width());

    for x in 0..imgb.width()
    {
        for y in 0..imgb.height()
        {
            imgb.put_pixel(x, y, img.get_pixel(y, x));
        }
    }

    *img = DynamicImage::ImageRgba8(imgb); //Making img point to processed image

    flip_vertically(&mut img);
}

fn blur_image(mut img : &mut DynamicImage, blur_strength : u32)
{
    for x in 0..(img.width() - blur_strength)
    {
        for y in 0..(img.height() - blur_strength)
        {
            let mut value : Rgba<u32> = Rgba([0, 0, 0, 0]);
            for j in 0..blur_strength
            {
                for k in 0..blur_strength
                {
                    let pixel = img.get_pixel(x + j, y + k);
                    value.0[0] += pixel.0[0] as u32;
                    value.0[1] += pixel.0[1] as u32;
                    value.0[2] += pixel.0[2] as u32;
                    
                }
            }
            //let mut value1 : Rgba<u8> = ;
            let divider = blur_strength * blur_strength;
            img.put_pixel(x, y, Rgba([(value.0[0] / (divider)) as u8, (value.0[1] / (divider)) as u8, (value.0[2] / (divider)) as u8, (value.0[3] / (divider)) as u8]));
        }
    }
}

