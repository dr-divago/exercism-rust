// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    to_string(level)+": "+message
}
pub fn info(message: &str) -> String {
    "[INFO]: ".to_string()+message
}
pub fn warn(message: &str) -> String {
    "[WARNING]: ".to_string()+message
}
pub fn error(message: &str) -> String {
    "[ERROR]: ".to_string()+message
}

fn to_string(level : LogLevel) -> String {
    match level {
        LogLevel::Error   => "[ERROR]".to_string(),
        LogLevel::Info    => "[INFO]".to_string(),
        LogLevel::Warning => "[WARNING]".to_string(),
        LogLevel::Debug   => "[DEBUG]".to_string()
    }
}
