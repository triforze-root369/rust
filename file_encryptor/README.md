# File Encryptor

A simple command-line file encryption program written in Rust that supports Caesar Cipher and XOR encryption methods.

## Features

- Two encryption methods:
  - Caesar Cipher: A basic substitution cipher that shifts characters by a specified amount
  - XOR Encryption: A symmetric encryption using a user-provided key
- User-friendly command-line interface
- Support for both text and binary files (XOR encryption)
- Secure metadata storage for decryption
- Error handling and input validation

## Prerequisites

- Rust and Cargo (latest stable version)

## Installation

1. Clone this repository or download the source code
2. Navigate to the project directory
3. Build the project:
```bash
cargo build --release
```

The compiled binary will be available in `target/release/file_encryptor`

## Usage

Run the program:
```bash
cargo run
```

Or use the compiled binary directly:
```bash
./target/release/file_encryptor
```

### Encrypting a File

1. Select option 1 (Encrypt File)
2. Enter the path to the file you want to encrypt
3. Choose the encryption method:
   - Option 1: Caesar Cipher (enter a shift value between 1-25)
   - Option 2: XOR Encryption (enter a key up to 32 characters)
4. The encrypted file will be saved with a `.enc` extension

### Decrypting a File

1. Select option 2 (Decrypt File)
2. Enter the path to the encrypted file (must have `.enc` extension)
3. The program will automatically detect the encryption method and parameters from the metadata
4. The decrypted file will be saved with a `.dec` extension

## Limitations

- Caesar Cipher is not secure for sensitive data (it's for educational purposes only)
- Maximum file size: 1GB
- XOR encryption key length: 32 characters maximum

## Example

```
Welcome to File Encryptor!

Please select an option:
1. Encrypt File
2. Decrypt File
3. Exit

1
Enter the path of the file to encrypt:
example.txt

Select encryption method:
1. Caesar Cipher
2. XOR Encryption

1
Enter shift value (1-25):
3

File encrypted successfully as example.txt.enc!
```

## Security Note

This program is designed for educational purposes and basic file encryption needs. The Caesar Cipher method is not secure for sensitive data. The XOR encryption method provides better security but still may not be suitable for highly sensitive information. For serious security needs, please use established encryption libraries and standards. 