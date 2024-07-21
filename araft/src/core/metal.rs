use serde::{Serialize, Deserialize};
use tempfile::NamedTempFile;
use thiserror::Error;
use std::io::Write;
use std::fs;

#[derive(Error, Debug)]
pub enum Err {
    #[error("Could not open file")]
    FileOpen,

    #[error("Could not write to file")]
    FileWrite,

    #[error("Could not save file")]
    FileStore,

    #[error("Could not read file")]
    FileRead,

    #[error("Could (de)serialize")]
    Format,

    #[error("unknown error")]
    Unknown,
}

pub fn write<T>(data: &T, path_file: std::path::PathBuf) -> Result<(), Err>
    where T: Serialize 
{
    let j = serde_json::to_string(data);
    let Ok(j) = j else {
        return Err(Err::Format);
    };

    let mut file = NamedTempFile::new();
    let Ok(mut file) = file else {
        return Err(Err::FileOpen);
    };

    let res = write!(file, "{}", j);
    if res.is_err() {
        return Err(Err::FileWrite);
    }

    let path = file.into_temp_path();
    let res = path.persist(path_file);
    if res.is_err() {
        return Err(Err::FileStore);
    }

    Ok(())
}

pub fn read<T>(path_file: std::path::PathBuf) -> Result<T, Err> 
    where T: for<'a> Deserialize<'a>
{
    let contents = fs::read_to_string(path_file);
    let Ok(contents) = contents else {
        return Err(Err::FileRead);
    };

    let d = serde_json::from_str::<T>(&contents);

    let Ok(d) = d else {
        return Err(Err::Format);
    };

    Ok(d)
}