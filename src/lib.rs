#![doc = include_str!("../README.md")]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![deny(unused_crate_dependencies)]

/// a loaded configuration, accessed by path
pub mod HConfig;
/// singleton holding the configs directory and the loaded configs
pub mod HConfigManager;
/// conversion helpers
pub mod Utils;
/// file formats (`IOwrapper` trait and its implementations)
pub mod IO;

pub extern crate tinyjson;

use thiserror::Error;
use tinyjson::JsonParseError;

/// errors of the library, each one carries the config name
#[derive(Error, Debug)]
pub enum Errors
{
	/// generic error (config name, message), available for custom `IOwrapper` implementations
	#[error("config {0} error : {1}")]
	AnyError(String, String),
	/// the config file is missing and cannot be created (config name, file path, cause)
	#[error("config {0} cannot create the file {1} : {2}")]
	ConfigCannotCreateFile(String, String, #[source] std::io::Error),
	/// the config file exists but cannot be read (config name, file path, cause)
	#[error("config {0} cannot load the file {1} : {2}")]
	ConfigCannotLoadFile(String, String, #[source] std::io::Error),
	/// the config file content is not valid json (config name, file path, cause)
	#[error("config {0} cannot convert the file content {1} into json : {2}")]
	ConfigCannotConvertFileToJsonValue(String, String, #[source] JsonParseError),
	/// the config cannot be written to its file (config name, file path, cause)
	#[error("config {0} cannot save the file content {1} : {2}")]
	ConfigCannotSaveFile(String, String, #[source] std::io::Error),
}
