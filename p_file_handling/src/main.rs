// Opening a File
// Reading a File
// Writing to a File
// Appending to a File
// Removing a File

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
use std::fs;

use std::{ thread, time };
fn main() {
    read_user_input();
}

fn file_open() {
    let file_open_result = File::open("Cargo.toml");
    // let no_open_result = File::open("nodata.text");

    match file_open_result {
        Ok(file) => {
            println!("File opened successfully: {:?}", file);
        }
        Err(error) => {
            println!("Error opening file: {:?}", error);
        }
    }
}

fn file_read() {
    let mut file = File::open("Cargo.toml").unwrap();

    let mut file_content = String::new();

    let num_bytes = file.read_to_string(&mut file_content).unwrap();

    println!("File contents: {}", file_content);
    println!("Number of bytes: {}", num_bytes);
}

fn file_write() {
    let mut file = File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .open("data.txt")
        .unwrap();

    // println!("{:?}", file);

    // "13th of July Class.".as_bytes()
    // b"13th of July Class."
    let res = file.write("Some different Message\n".as_bytes());
    println!("Number of bytes written to the file: {:?}", res.unwrap());
}

fn file_append() {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("data.txt")
        .unwrap();

    let res = file.write("Successfully appended to this file.\n".as_bytes());
    println!("Number of bytes written to the file: {:?}", res.unwrap());
}

fn file_remove() {
    let removal_result = fs::remove_file("data.txt");
    println!("{:?}", removal_result);
}

fn read_write() {
    let mut file = File::options()
        .read(true)
        .write(true)
        .open("data.txt")
        .unwrap();

    let mut file_content = String::new();

    let num_bytes = file.read_to_string(&mut file_content).unwrap();

    println!("File contents: {}", file_content);
    println!("1st Number of bytes: {}", num_bytes);

    thread::sleep(time::Duration::from_millis(1000));

    let res = file.write("Appending???\n".as_bytes());
    println!("Number of bytes written to the file: {:?}", res.unwrap());

    thread::sleep(time::Duration::from_millis(3000));

    let mut file_content_second = String::new();
    let num_bytes = file.read_to_string(&mut file_content_second).unwrap();

    println!("File contents: {}", file_content_second);
    println!("2nd Number of bytes: {}", num_bytes);
}

fn read_user_input() {
    let mut input = String::new(); 
    println!("Enter your name: ");
    let input_bytes = std::io::stdin().read_line(&mut input).unwrap(); // Actually reading the input from the input stream(stdin)
    println!("Your name is: {}", input);
    println!("Number of bytes: {}\n\n", input_bytes);
}
