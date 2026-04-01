use std::{fmt::Display, io::Error};

use serde_json::Value;
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};

async fn read_and_parse(file_name: &str) -> String {
    let mut f = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.unwrap();
    let s = String::from_utf8(buf).unwrap();

    let v: Value = serde_json::from_str(s.as_str()).unwrap();
    v.get("connectionString").unwrap().to_string()
}

async fn read_and_parse_2(file_name: &str) -> Result<String, &str> {
    let mut f = File::open(file_name).await.map_err(|_| "failed to open file")?; // error propagation
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.map_err(|_| "failed to read file")?;
    let s = String::from_utf8(buf).map_err(|_| "failed to parse UTF-8")?;    
    
    let v: Value = serde_json::from_str(s.as_str()).map_err(|_| "failed to parse JSON")?;
    match v.get("connectionString") {
        Some(value) => Ok(value.to_string()),
        None => Err("connectionString not found in JSON"),
    }
}

async fn read_and_parse_3(file_name: &str) -> anyhow::Result<String> {
    let mut f = File::open(file_name).await?; // error propagation
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let s = String::from_utf8(buf)?;    
    
    let v: Value = serde_json::from_str(s.as_str())?;
    match v.get("connectionString") {
        Some(value) => Ok(value.to_string()),
         None => Err(anyhow::anyhow!("connectionString not found in JSON")),    }
}

#[derive(Error, Debug)]
enum ReadAndParseError {
    #[error("failed to open file ({0})")]
    FileError(#[from] std::io::Error),
    #[error("failed to read file")]
    Utf8ParseError(#[from] std::string::FromUtf8Error),
    #[error("failed to parse UTF-8")]
    JsonParseError(#[from] serde_json::Error),
    #[error("connectionString not found in JSON")]
    ConnectionStringNotFound,
}

async fn read_and_parse_4(file_name: &str) -> Result<String, ReadAndParseError> {
    let mut f = File::open(file_name).await?; // error propagation
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let s = String::from_utf8(buf)?;    
    
    let v: Value = serde_json::from_str(s.as_str())?;
    match v.get("connectionString") {
        Some(value) => Ok(value.to_string()),
        None => Err(ReadAndParseError::ConnectionStringNotFound),
    }
}

#[tokio::main]
async fn main() {
    let connection_string = read_and_parse_4("conig.json").await;
    match connection_string {
        Ok(conn_str) => println!("Connection string: {}", conn_str),
        Err(e) => println!("Error: {}", e),
    }
}