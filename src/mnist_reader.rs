extern crate nalgebra;
extern crate byteorder;
extern crate pbr;
extern crate typenum;

use self::pbr::ProgressBar;
use std::time::Duration;
use std::io::Cursor;
use std::fs::File;
use std::io::prelude::*;
use self::nalgebra::core::{DVector};
use self::byteorder::{BigEndian, ReadBytesExt};

pub fn read(image_file_name: &str, label_file_name: &str) -> Result<Vec<MnistImage>, String> {
    let image_file = File::open(image_file_name);
    let mut images: Vec<MnistImage>;

    match image_file {
        Ok(image_file) => {
            match read_image_file(image_file) {
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
    
    let label_file = File::open(label_file_name);
    match label_file {
        Ok(label_file) => {
            match read_label_file(label_file, &mut images) {
                Ok(_) => {},
                Err(msg) => {
                    return Err(format!(
                        "Couldn't read labels from file '{}'! Error - {}",
                        label_file_name,
                        msg
                    ))
                }
            }
        }
        _ => return Err(format!("No such file '{}'!", label_file_name)),
    }

    Ok(images)
}

fn read_image_file(file: File) -> Result<Vec<MnistImage>, String> {
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

    println!("Reading images");
    let mut pb = ProgressBar::new(images.capacity() as u64);
    pb.set_max_refresh_rate(Some(Duration::from_millis(250)));
    for _ in 0..images.capacity() {
        let mut vector = MnistVector::from_element(28*28, 0.);
        pb.inc();
        match read_28x28_image(&file) {
            Ok(data) => {
                for i in 0..28*28 {
                    vector[i] = f64::from(data[i]);
                }
            }
            Err(_) => {}
        }
        let image = MnistImage::new(vector, 0);
        images.push(image);
    }
    pb.finish_println("");

    Ok(images)
}

//Adds labels to already read images
fn read_label_file(file: File, images: &mut Vec<MnistImage>) -> Result<(), String> {
    match read_u32(&file) {
        Ok(magic_number) => {
            if magic_number == 2051 {
                return Err(format!("File mismatch. This is image file, not label!"));
            } else if magic_number != 2049 {
                return Err(format!(
                    "Magic number is wrong! Expected 2049, got {}",
                    magic_number
                ));
            }
        }
        Err(msg) => return Err(format!("Cannot read magic number! Error - {}", msg)),
    }
    
    match read_u32(&file) {
        Ok(images_count) => if images.len() != images_count as usize {
            return Err(format!("Labels count does not match images! Labels count - {}, images count - {}", images.len(), images_count));
        },
        _ => return Err(format!("Cannot read images count!")),
    }

    println!("Reading labels");
    let mut pb = ProgressBar::new(images.capacity() as u64);
    pb.set_max_refresh_rate(Some(Duration::from_millis(250)));
    for i in 0..images.capacity() {
        match read_u8(&file) {
            Ok(label) => {
                images[i].label = label as usize;
            }
            Err(_) => {}
        }
    }
    pb.finish_println("");

    println!("\nDiagnostic print, 100, 101, 102 and 50000 images:");
    images[100].draw();
    images[101].draw();
    images[102].draw();
    images[50000].draw();

    Ok(())
}

/// It might need to be i32
fn read_u32(mut file: &File) -> Result<u32, String> {
    let mut buffer = &mut [0; 4];

    match file.read_exact(buffer) {
        Ok(()) => {
            let mut cursor = Cursor::new(&buffer);
            Ok(cursor.read_u32::<BigEndian>().unwrap())
        }
        Err(e) => Err(format!("File read error. Error - {}", e)),
    }
}

fn read_u8(mut file: &File) -> Result<u8, String> {
    let mut buffer = &mut [0; 1];

    match file.read_exact(buffer) {
        Ok(()) => {
            let mut cursor = Cursor::new(&buffer);
            Ok(cursor.read_u8().unwrap())
        }
        Err(e) => Err(format!("File read error. Error - {}", e)),
    }
}

fn read_28x28_image(mut file: &File) -> Result<[u8; 28 * 28], String> {
    let buffer = &mut [0; 28 * 28];

    match file.read_exact(buffer) {
        Ok(()) => {
            Ok(*buffer)
        }
        Err(e) => Err(format!("File read error. Error - {}", e)),
    }
}

pub type MnistVector = DVector<f64>;

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
        println!("Label - {}", self.label);
        for i in 0..28*28 {
            if i % 28 == 0 {
                println!("");
            }
            match self.data[i] {
                v if v < 10. => print!(" "),
                v if v < 64. => print!("░"),
                v if v < 128. => print!("▒"),
                v if v < 192. => print!("▓"),
                _ => print!("█")
            }   
        }
        println!("");
    }
}