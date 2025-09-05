use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::bb8::RunError as BB8Error;
use serenity::prelude::SerenityError;
use std::env::VarError;
use std::fmt;
use std::num::TryFromIntError;

#[derive(Debug)]
pub enum ClientError {
    SerenityError(String),
    BB8Error(String),
    DieselError(String),
    ReqwestError(String),
    VarError(String),
    TryFromIntError(String),
    YmlError(String),
    IoError(String),
    JsonError(String),
    OtherStatic(&'static str),
    Other(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SerenityError(error) => write!(f, "**Serenity erreur: {error}**"),
            Self::BB8Error(error) => write!(f, "**bb8 erreur: {error}**"),
            Self::DieselError(error) => write!(f, "**diesel erreur: {error}**"),
            Self::ReqwestError(error) => write!(f, "**reqwest erreur: {error}**"),
            Self::VarError(error) => write!(f, "**missing env var: {error}**"),
            Self::TryFromIntError(error) => {
                write!(f, "**integer type conversion erreur: {error}**")
            }
            Self::YmlError(error) => write!(f, "**yml conversion erreur: {error}**"),
            Self::IoError(error) => write!(f, "**file erreur: {error}**"),
            Self::JsonError(error) => write!(f, "**parse json file ereur: {error}**"),
            Self::OtherStatic(error) => write!(f, "**Erreur: {error}**"),
            Self::Other(error) => write!(f, "**Erreur: {error}**"),
        }
    }
}

impl std::error::Error for ClientError {}

impl From<SerenityError> for ClientError {
    fn from(error: SerenityError) -> Self {
        Self::SerenityError(error.to_string())
    }
}

impl From<BB8Error> for ClientError {
    fn from(error: BB8Error) -> Self {
        Self::BB8Error(error.to_string())
    }
}

impl From<DieselError> for ClientError {
    fn from(error: DieselError) -> Self {
        Self::DieselError(error.to_string())
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error.to_string())
    }
}

impl From<VarError> for ClientError {
    fn from(error: VarError) -> Self {
        Self::VarError(error.to_string())
    }
}

impl From<TryFromIntError> for ClientError {
    fn from(error: TryFromIntError) -> Self {
        Self::TryFromIntError(error.to_string())
    }
}

impl From<serde_yml::Error> for ClientError {
    fn from(error: serde_yml::Error) -> Self {
        Self::YmlError(error.to_string())
    }
}

impl From<std::io::Error> for ClientError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error.to_string())
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(error.to_string())
    }
}
