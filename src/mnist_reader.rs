extern crate nalgebra;
extern crate byteorder;
extern crate pbr;
extern crate typenum;

use self::pbr::ProgressBar;
use std::io::Cursor;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use self::nalgebra::core::{Matrix, MatrixArray, VectorN};
use self::typenum::U784;
use self::byteorder::{BigEndian, ReadBytesExt};
use std::time::Duration;

pub fn read(image_file_name: &str, label_file_name: &str) -> Result<Vec<MnistImage>, String> {
    let file = File::open(image_file_name);

    let images: Vec<MnistImage>;

    match file {
        Ok(file) => {
            match read_image_file(file) {
                Ok(result) => images = result,
                Err(msg) => {
                    return Err(format!(
                        "Couldn't read images from file '{}'! Error - {}",
                        image_file_name,
                        msg
                    ))
                }
            }
        }
        _ => return Err(format!("No such file '{}'!", image_file_name)),
    }

    Ok(images)
}

fn read_image_file(mut file: File) -> Result<Vec<MnistImage>, String> {

    match read_u32(&file) {
        Ok(magic_number) => {
            if magic_number == 2049 {
                return Err(format!("File mismatch. This is label file, not image!"));
            } else if magic_number != 2051 {
                return Err(format!(
                    "Magic number is wrong! Expected 2051, got {}",
                    magic_number
                ));
            }
        }
        Err(msg) => return Err(format!("Cannot read magic number! Error - {}", msg)),
    }

    let mut images: Vec<MnistImage>;

    match read_u32(&file) {
        Ok(images_count) => images = Vec::with_capacity(images_count as usize),
        _ => return Err(format!("Cannot read images count!")),
    }

    match read_u32(&file) {
        Ok(rows_count) => {
            if rows_count != 28 {
                return Err(format!(
                    "Rows count not equal to 28, got {}. Only 28x28 images supported!",
                    rows_count
                ));
            }
        }
        _ => return Err(format!("Cannot read rows count!")),
    }

    match read_u32(&file) {
        Ok(columns_count) => {
            if columns_count != 28 {
                return Err(format!(
                    "Columns count not equal to 28, got {}. Only 28x28 images supported!",
                    columns_count
                ));
            }
        }
        _ => return Err(format!("Cannot read columns count!")),
    }

    let mut pb = ProgressBar::new(images.capacity() as u64);
    pb.set_max_refresh_rate(Some(Duration::from_millis(250)));
    for i in 0..images.capacity() {
        let mut vector = MnistVector::from_element(0);
        pb.inc();
        match read_28x28_image(&file) {
            Ok(data) => {
                for i in 0..28*28 {
                    vector[i] = data[i];
                }
            }
            Err(msg) => {}
        }
        // match read_28x28_image(&file) {
        //     Ok(data) => {
        //         for y in 0..28 {
        //             for x in 0..28 {
        //                 matrix[(x, y)] = data[x + y * 28];
        //             }
        //         }
        //     }
        //     Err(msg) => Err(format!("Can't read {}. image! Error - {}", i, msg)),
        // }
        let image = MnistImage::new(vector, 0);
        images.push(image);
    }
    pb.finish();

    println!("Diagnostic print, 100, 2001 and 50000 images:");
    images[100].draw();
    images[2001].draw();
    images[50000].draw();

    Ok(images)
}

// fn read_label_file(mut file: File) -> Vec<MnistImage> {

//     let mut buffer: &mut [u8] = &mut [];

//     match file.read(buffer) {
//         Ok(magic_number) => {
//             if magic_number == 2051 {
//                 return Err("File mismatch. This is image file, not label!");
//             }
//             else if magic_number != 2049 {
//                 return Err("Magic number is wrong!");
//             }
//         }
//         _ => return Err("Cannot read magic number!")
//     }

//     unimplemented!();
// }

/// It might need to be i32
fn read_u32(mut file: &File) -> Result<u32, String> {
    let mut buffer = &mut [0; 4];

    match file.read_exact(buffer) {
        Ok(()) => {
            let mut cursor = Cursor::new(&buffer);
            Ok(cursor.read_u32::<BigEndian>().unwrap())
        }
        Err(e) => Err(format!("File read error")),
    }
}

fn read_u8(mut file: &File) -> Result<u8, String> {
    let mut buffer = &mut [0; 1];

    match file.read_exact(buffer) {
        Ok(()) => {
            let mut cursor = Cursor::new(&buffer);
            Ok(cursor.read_u8().unwrap())
        }
        Err(e) => Err(format!("File read error")),
    }
}

fn read_28x28_image(mut file: &File) -> Result<[u8; 28 * 28], String> {
    let buffer = &mut [0; 28 * 28];

    match file.read_exact(buffer) {
        Ok(()) => {
            Ok(*buffer)
        }
        Err(e) => Err(format!("File read error")),
    }
}

type MnistVector = VectorN<u8, U784>;

#[derive(Debug)]
pub struct MnistImage {
    pub data: MnistVector,
    pub label: usize,
}

impl MnistImage {
    pub fn new(data: MnistVector, label: usize) -> MnistImage {
        MnistImage {
            data: data,
            label: label,
        }
    }

    pub fn draw(&self) {
        for i in 0..28*28 {
            if i % 28 == 0 {
                println!("");
            }
            match self.data[i] {
                v if v < 10 => print!(" "),
                v if v < 64 => print!("░"),
                v if v < 128 => print!("▒"),
                v if v < 192 => print!("▓"),
                _ => print!("█")
            }   
        }
        println!("");
    }
}