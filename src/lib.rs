#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_parens)]
#![deny(unused_crate_dependencies)]

pub mod HConfig;
pub mod HConfigManager;
pub mod Utils;
pub mod guard;

pub extern crate serde_json;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors
{
	#[error("config path must be set via 'setConfPath'")]
	ConfigNotSet,
	#[error("config {0} cannot create the file {1} : {2}")]
	ConfigCannotCreateFile(String, String, #[source] std::io::Error),
	#[error("config {0} cannot convert the file content {1} into json : {2}")]
	ConfigCannotConvertFileToJsonValue(String, String, #[source] serde_json::Error),
	#[error("config {0} cannot save the file content {1} : {2}")]
	ConfigCannotSaveFile(String, String, #[source] std::io::Error),
	#[error("config {0} cannot save the file content, json error {1} : {2}")]
	ConfigCannotSaveFileSerde(String, String, #[source] serde_json::Error)
}
