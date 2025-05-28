use std::fs::{self, File};
use std::io::{self, Read, Write, BufReader, BufWriter, BufRead};
use std::path::Path;
use serde::{Serialize, Deserialize};

// Metadata structure for storing encryption information
#[derive(Serialize, Deserialize)]
struct EncryptionMetadata {
    method: String,
    shift: Option<i32>,      // For Caesar cipher
    key: Option<String>,     // For XOR encryption
}

// Main menu options
enum MenuOption {
    Encrypt,
    Decrypt,
    Exit,
}

// Encryption methods
enum EncryptionMethod {
    Caesar,
    Xor,
}

fn main() {
    println!("Welcome to File Encryptor!");
    
    loop {
        match display_main_menu() {
            MenuOption::Encrypt => handle_encryption(),
            MenuOption::Decrypt => handle_decryption(),
            MenuOption::Exit => {
                println!("Thank you for using File Encryptor!");
                break;
            }
        }
    }
}

fn display_main_menu() -> MenuOption {
    println!("\nPlease select an option:");
    println!("1. Encrypt File");
    println!("2. Decrypt File");
    println!("3. Exit");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => return MenuOption::Encrypt,
            "2" => return MenuOption::Decrypt,
            "3" => return MenuOption::Exit,
            _ => println!("Invalid option. Please try again."),
        }
    }
}

fn get_encryption_method() -> EncryptionMethod {
    println!("\nSelect encryption method:");
    println!("1. Caesar Cipher");
    println!("2. XOR Encryption");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => return EncryptionMethod::Caesar,
            "2" => return EncryptionMethod::Xor,
            _ => println!("Invalid option. Please try again."),
        }
    }
}

fn get_file_path(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn handle_encryption() {
    let input_path = get_file_path("Enter the path of the file to encrypt:");
    if !Path::new(&input_path).exists() {
        println!("Error: File not found!");
        return;
    }

    let method = get_encryption_method();
    let metadata = match method {
        EncryptionMethod::Caesar => {
            let shift = get_caesar_shift();
            EncryptionMetadata {
                method: "caesar".to_string(),
                shift: Some(shift),
                key: None,
            }
        },
        EncryptionMethod::Xor => {
            let key = get_xor_key();
            EncryptionMetadata {
                method: "xor".to_string(),
                shift: None,
                key: Some(key),
            }
        },
    };

    let output_path = format!("{}.enc", input_path);
    
    match encrypt_file(&input_path, &output_path, &metadata) {
        Ok(_) => println!("File encrypted successfully as {}!", output_path),
        Err(e) => println!("Error during encryption: {}", e),
    }
}

fn handle_decryption() {
    let input_path = get_file_path("Enter the path of the encrypted file:");
    if !Path::new(&input_path).exists() {
        println!("Error: File not found!");
        return;
    }

    let output_path = input_path.replace(".enc", ".dec");
    
    match decrypt_file(&input_path, &output_path) {
        Ok(_) => println!("File decrypted successfully as {}!", output_path),
        Err(e) => println!("Error during decryption: {}", e),
    }
}

fn get_caesar_shift() -> i32 {
    loop {
        println!("Enter shift value (1-25):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        if let Ok(shift) = input.trim().parse::<i32>() {
            if shift >= 1 && shift <= 25 {
                return shift;
            }
        }
        println!("Invalid shift value. Please enter a number between 1 and 25.");
    }
}

fn get_xor_key() -> String {
    loop {
        println!("Enter encryption key (max 32 characters):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let key = input.trim().to_string();
        if key.len() > 0 && key.len() <= 32 {
            return key;
        }
        println!("Invalid key. Please enter 1-32 characters.");
    }
}

fn encrypt_file(input_path: &str, output_path: &str, metadata: &EncryptionMetadata) -> io::Result<()> {
    let input_data = fs::read(input_path)?;
    let encrypted_data = match metadata.method.as_str() {
        "caesar" => caesar_encrypt(&input_data, metadata.shift.unwrap()),
        "xor" => xor_encrypt(&input_data, metadata.key.as_ref().unwrap()),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid encryption method")),
    };

    let mut output_file = BufWriter::new(File::create(output_path)?);
    
    // Write metadata
    let metadata_json = serde_json::to_string(metadata)?;
    writeln!(output_file, "{}", metadata_json)?;
    
    // Write encrypted data
    output_file.write_all(&encrypted_data)?;
    output_file.flush()?;
    
    Ok(())
}

fn decrypt_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let mut reader = BufReader::new(File::open(input_path)?);
    
    // Read metadata
    let mut metadata_line = String::new();
    reader.read_line(&mut metadata_line)?;
    
    let metadata: EncryptionMetadata = serde_json::from_str(&metadata_line)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    // Read encrypted data
    let mut encrypted_data = Vec::new();
    reader.read_to_end(&mut encrypted_data)?;
    
    let decrypted_data = match metadata.method.as_str() {
        "caesar" => caesar_decrypt(&encrypted_data, metadata.shift.unwrap()),
        "xor" => xor_decrypt(&encrypted_data, metadata.key.as_ref().unwrap()),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid encryption method")),
    };
    
    fs::write(output_path, decrypted_data)?;
    Ok(())
}

fn caesar_encrypt(data: &[u8], shift: i32) -> Vec<u8> {
    data.iter()
        .map(|&byte| {
            if byte.is_ascii_alphabetic() {
                let base = if byte.is_ascii_uppercase() { b'A' } else { b'a' };
                let shifted = (byte - base + shift as u8) % 26 + base;
                shifted
            } else {
                byte
            }
        })
        .collect()
}

fn caesar_decrypt(data: &[u8], shift: i32) -> Vec<u8> {
    // For Caesar cipher, decryption is just encryption with the opposite shift
    caesar_encrypt(data, 26 - shift)
}

fn xor_encrypt(data: &[u8], key: &str) -> Vec<u8> {
    let key_bytes = key.as_bytes();
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key_bytes[i % key_bytes.len()])
        .collect()
}

fn xor_decrypt(data: &[u8], key: &str) -> Vec<u8> {
    // XOR encryption is its own inverse when using the same key
    xor_encrypt(data, key)
}
