use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use thiserror::Error;
use colored::Colorize;

const AWS_ACCESS_KEY_ID_NAME: &str = "AWS_ACCESS_KEY_ID";
const AWS_SECRET_ACCESS_KEY_NAME: &str = "AWS_SECRET_ACCESS_KEY";
const AWS_SESSION_TOKEN_NAME: &str = "AWS_SESSION_TOKEN";
const HOME_NAME: &str = "HOME";

#[derive(Error, Debug)]
pub enum AppError {
    #[error("An unknown error occurred")]
    Unknown,

    #[error("Can not parse variable: {input}")]
    Var {
        input: &'static str,
        #[source] 
        source: std::env::VarError,
    },

    #[error("Can not open credentials file: {input}")]
    Io{
        input: String,
        #[source] 
        source: std::io::Error,
    },
}

fn main() {

    match run() {
        Ok(value) => println!("{} {} {}", "credential file".green(), value.bold().bright_green(), "is updated".green()),
        Err(err) => eprintln!("{} {}", "error:".red(), err),
    }
}

fn run() -> Result<String, AppError> {

    let home_path = env::var(HOME_NAME)
        .map_err(|e| AppError::Var { input: HOME_NAME, source: e })?;

    let aws_access_key = env::var(AWS_ACCESS_KEY_ID_NAME)
        .map_err(|e| AppError::Var { input: AWS_ACCESS_KEY_ID_NAME, source: e })?;
    let aws_secret_key = env::var(AWS_SECRET_ACCESS_KEY_NAME)
        .map_err(|e| AppError::Var { input: AWS_SECRET_ACCESS_KEY_NAME, source: e })?;
    let aws_session_token = env::var(AWS_SESSION_TOKEN_NAME)
        .map_err(|e| AppError::Var { input: AWS_SESSION_TOKEN_NAME, source: e })?;

    let file_path = home_path + "/.aws/credentials";

    let mut file = OpenOptions::new().write(true).append(false).open(&file_path)
                .map_err(|e| AppError::Io { input: file_path.clone(), source: e })?;

    let mut str = String::new();
    str.push_str("[default]\n");
    str.push_str(&format!("aws_access_key_id={}\n", aws_access_key));
    str.push_str(&format!("aws_secret_access_key={}\n", aws_secret_key));
    str.push_str(&format!("aws_session_token={}", aws_session_token));

    file.write_all(str.as_bytes())
            .map_err(|e| AppError::Io { input: file_path.clone(), source: e })?;
    
    Ok(file_path)
}