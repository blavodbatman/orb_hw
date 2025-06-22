use regex::Regex;
use thiserror::Error;
use ring::{pbkdf2, rand};
use std::num::NonZeroU32;
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use std::{fs::{self, OpenOptions}, io::{self, BufRead, Write}};
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};

pub static FILE_NAME: &str = "passman";
static HELP_TEXT: &str = "Available commands: exit, help, add, show, show all, change, delete, delete all, delete file.";

const SALT_AND_HASH_LEN: usize = 64;
const CRYPTO_KEY_LEN: usize = 32;
const INIT_VEC_LEN: usize = 16;

#[derive(Error, Debug)]
pub enum Errors {
	#[error("Can't open file")]
	FileOpen,
	#[error("Can't remove file")]
	FileRemove,
	#[error("Invalid password format")]
	InvalidFormat,
	#[error("IO error")]
	IO(#[from] io::Error),
	#[error("Data is malformed")]
	CorruptedData,
	#[error("Resource not found")]
	NotFound,
	#[error("Can't write to file")]
	FileWrite,
	#[error("Unspecified error")]
	Unspecified,
}

pub type Result<T> = std::result::Result<T, Errors>;

pub fn exit() {
	println!("See you later!");
}

pub fn help() {
	println!("{HELP_TEXT}");
}

pub fn add(mpass: &str) -> Result<()> {
	let resource = type_name();
	if find(&resource, mpass).is_ok() {
		println!("Record for resource {} already exists. Use change command.", resource);
		return Ok(());
	}
	let password = type_password("Enter password:");
	add_record(&resource, &password, mpass)?;
	Ok(())
}

pub fn show(mpass: &str) -> Result<()> {
	let resource = type_name();
	if let Ok((_, pass)) = find(&resource, mpass) {
		println!("{pass}");
	} else {
		println!("Resource not found.");
	}
	Ok(())
}

pub fn show_all(mpass: &str) -> Result<()> {
	let mut file = OpenOptions::new().read(true).open(FILE_NAME).map_err(|_| Errors::FileOpen)?;
    let mut reader = io::BufReader::new(file);
	if reader.lines().count() == 1 {
		println!("Resources list is empty.");
		return Ok(());
	}
    
	file = OpenOptions::new().read(true).open(FILE_NAME).map_err(|_| Errors::FileOpen)?;
    reader = io::BufReader::new(file);
    for line in reader.lines().skip(1) {
        let line = line.map_err(|_| Errors::CorruptedData)?;
		let decrypted_data = decrypt_data(line, &generate_crypto_key(mpass))?;
		println!("{}", decrypted_data);
    }
	Ok(())
}

pub fn change(mpass: &str) -> Result<()> {
	let resource = type_name();
	if let Ok((index, pass)) = find(&resource, mpass) {
		let password = type_password("Enter new password:");
		if password == pass {
			println!("It's the same password.");
			return Ok(());
		}
		delete_record(index)?;
		add_record(&resource, &password, mpass)?;
	} else {
		println!("Resource not found.");
	}
	Ok(())
}

pub fn delete(mpass: &str) -> Result<()> {
	let resource = type_name();
	if let Ok((index, _)) = find(&resource, mpass) {
		delete_record(index)?;
	} else {
		println!("Resource not found.");
	}
	Ok(())
}

pub fn delete_all() -> Result<()> {
	let mut file = OpenOptions::new().read(true).open(FILE_NAME).map_err(|_| Errors::FileOpen)?;
	let reader = io::BufReader::new(&file);
	let line = reader.lines().next().unwrap().map_err(|_| Errors::CorruptedData)?;
	file = OpenOptions::new().write(true).truncate(true).open(FILE_NAME).map_err(|_| Errors::FileOpen)?;
	writeln!(file, "{}", line).map_err(|_| Errors::FileWrite)?;
	Ok(())
}

pub fn delete_file() -> Result<()> {
	fs::remove_file(FILE_NAME).map_err(|_| Errors::FileRemove)?;
	println!("See you later!");
	Ok(())
}


pub fn type_password(message: &str) -> String {
	let password: String;
	loop {
		println!("{message}");
		if let Ok(pass) = check_password() {
			password = pass;
			break;
		};
	}
	password
}

pub fn save_mpassword(mpass: &str) -> Result<()> {
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; SALT_AND_HASH_LEN];
    rng.fill(&mut salt).map_err(|_| Errors::Unspecified)?;

    let mut pbkdf2_hash = [0u8; SALT_AND_HASH_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        mpass.as_bytes(),
        &mut pbkdf2_hash,
    );
	insert(&HEXUPPER.encode(&salt), &HEXUPPER.encode(&pbkdf2_hash))?;
	Ok(())
}

pub fn check_mpassword(pass: &str) -> Result<bool> {
	let file = OpenOptions::new().read(true).open(FILE_NAME).map_err(|_| Errors::FileOpen)?;
    let reader = io::BufReader::new(file);
	let line = reader.lines().next().unwrap().map_err(|_| Errors::CorruptedData)?;
	let (salt, pbkdf2_hash) = line.split_once(" ").ok_or(Errors::CorruptedData)?;
    let value = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(100_000).unwrap(),
        &HEXUPPER.decode(salt.as_bytes()).map_err(|_| Errors::Unspecified)?[..],
        pass.as_bytes(),
        &HEXUPPER.decode(pbkdf2_hash.as_bytes()).map_err(|_| Errors::Unspecified)?[..],
    );
	Ok(value.is_ok())
}

fn check_password() -> Result<String> {
	let pass = rpassword::read_password().map_err(|_| Errors::InvalidFormat)?;
	if !Regex::new(r"^[0-9A-Za-z]{6,20}$").unwrap().is_match(&pass) {
		println!("Password must consist of numbers and Latin letters, from 6 to 20 characters.");
		return Err(Errors::InvalidFormat);
	}
	Ok(pass)
}

fn type_name() -> String {
	let resource;
	let re = Regex::new(r"^[0-9A-Za-z@\-\.]+$").unwrap();
	loop {
		println!("Enter resource name:");
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		input = input.trim().to_string();
		if !re.is_match(&input) {
			println!("Resource name must consist of numbers, Latin letters and symbols: '.', '-' and '@'.");
			input.clear();
		} else {
			resource = input.to_lowercase();
			break;
		}
	}
	resource
}

fn add_record(resource: &str, password: &str, mpass: &str) -> Result<()> {
	let rng = rand::SystemRandom::new();
	let mut init_vec = [0u8; INIT_VEC_LEN];
	rng.fill(&mut init_vec).map_err(|_| Errors::Unspecified)?;
	let encrypted_data = encrypt(format!("{} {}", resource, password).as_bytes(), &generate_crypto_key(mpass), &init_vec).map_err(|_| Errors::Unspecified)?;
	insert(&HEXUPPER.encode(&init_vec), &HEXUPPER.encode(&encrypted_data))?;
	Ok(())
}

fn delete_record(index: usize) -> Result<()> {	
	let mut file = OpenOptions::new().read(true).open(FILE_NAME).map_err(|_| Errors::FileOpen)?;
	let reader = io::BufReader::new(file);
	let mut lines: Vec<String> = Vec::new();
	for (_, line) in reader.lines().enumerate().filter(|(i, _)| *i != index) {
		let line = line.map_err(|_| Errors::CorruptedData)?;
		lines.push(line.trim().to_string());
	}
	
	file = OpenOptions::new().write(true).truncate(true).open(FILE_NAME).map_err(|_| Errors::FileOpen)?;
	for line in lines.into_iter() {
		writeln!(file, "{}", line).map_err(|_| Errors::FileWrite)?;
	}
	Ok(())
}

fn insert(key: &str, value: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).open(FILE_NAME).map_err(|_| Errors::FileOpen)?;
    writeln!(file, "{} {}", key, value).map_err(|_| Errors::FileWrite)?;
	Ok(())
}

fn find(resource: &str, mpass: &str) -> Result<(usize, String)> {
    let file = OpenOptions::new().read(true).open(FILE_NAME).map_err(|_| Errors::FileOpen)?;
    let reader = io::BufReader::new(file);
    
    for (index, line) in reader.lines().enumerate().skip(1) {
        let line = line.map_err(|_| Errors::CorruptedData)?;
		let decrypted_data = decrypt_data(line, &generate_crypto_key(mpass))?;
		let (f_resource, f_password) = decrypted_data.split_once(" ").ok_or(Errors::CorruptedData)?;
		if f_resource == resource {
            return Ok((index, f_password.to_string()));
        }
    }
    Err(Errors::NotFound)
}

fn generate_crypto_key(mpass: &str) -> Vec<u8> {
	let mut arr = Vec::<u8>::with_capacity(CRYPTO_KEY_LEN);
	let mpass_bytes = mpass.as_bytes();
	arr.append(&mut mpass_bytes.to_vec());
    arr.extend(std::iter::repeat_n(0u8, CRYPTO_KEY_LEN - mpass_bytes.len()));
    arr
}

fn encrypt(data: &[u8], key: &[u8], init_vec: &[u8]) -> Result<Vec<u8>> {
    Ok(cbc::Encryptor::<aes::Aes256>::new(key.into(), init_vec.into()).encrypt_padded_vec_mut::<Pkcs7>(data))
}

fn decrypt(data: &[u8], key: &[u8], init_vec: &[u8]) -> Result<Vec<u8>> {
	cbc::Decryptor::<aes::Aes256>::new(key.into(), init_vec.into()).decrypt_padded_vec_mut::<Pkcs7>(data).map_err(|_| Errors::Unspecified)
}

fn decrypt_data(data: String, key: &[u8]) -> Result<String> {
	let (init_vec, value) = data.split_once(" ").ok_or(Errors::CorruptedData)?;
	let data = decrypt(&HEXUPPER.decode(value.as_bytes()).map_err(|_| Errors::Unspecified)?, key, &HEXUPPER.decode(init_vec.as_bytes()).map_err(|_| Errors::Unspecified)?).map_err(|_| Errors::Unspecified)?;
	Ok(String::from_utf8_lossy(&data).to_string())
}
