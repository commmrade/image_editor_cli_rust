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
    scale_down(&mut img, 10);
    rotate_image(&mut img);

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

