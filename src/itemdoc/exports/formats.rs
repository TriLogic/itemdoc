use super::core::{ItemOutput, ExportError};
use super::json::allman::*;
use super::json::compact::*;
use super::json::knr::*;
use super::json::linear::*;
use super::json::whitesmith::*;
use super::yaml::yaml::*;

pub enum OutputFormats {
    Compact(JSONFormatCompact),
    Linear(JSONFormatLinear),
    KNR(JSONFormatKNR),
    Allman(JSONFormatAllman),
    Whitesmith(JSONFormatWhitesmith),
    YAML(YAMLFormat)
}

impl ItemOutput for OutputFormats {

    fn list_open(&mut self) -> Result<usize, ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.list_open(),
            OutputFormats::Linear(fmt) => fmt.list_open(),
            OutputFormats::KNR(fmt) => fmt.list_open(),
            OutputFormats::Allman(fmt) => fmt.list_open(),
            OutputFormats::Whitesmith(fmt) => fmt.list_open(),
            OutputFormats::YAML(fmt) => fmt.list_open(),
        }
    }
    fn list_begin_next(&mut self) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.list_begin_next(),
            OutputFormats::Linear(fmt) => fmt.list_begin_next(),
            OutputFormats::KNR(fmt) => fmt.list_begin_next(),
            OutputFormats::Allman(fmt) => fmt.list_begin_next(),
            OutputFormats::Whitesmith(fmt) => fmt.list_begin_next(),
            OutputFormats::YAML(fmt) => fmt.list_begin_next(),
        }
    }
    fn list_write_null(&mut self) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.list_write_null(),
            OutputFormats::Linear(fmt) => fmt.list_write_null(),
            OutputFormats::KNR(fmt) => fmt.list_write_null(),
            OutputFormats::Allman(fmt) => fmt.list_write_null(),
            OutputFormats::Whitesmith(fmt) => fmt.list_write_null(),
            OutputFormats::YAML(fmt) => fmt.list_write_null(),
        }
    }
    fn list_write_bool(&mut self, value: bool) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.list_write_bool(value),
            OutputFormats::Linear(fmt) => fmt.list_write_bool(value),
            OutputFormats::KNR(fmt) => fmt.list_write_bool(value),
            OutputFormats::Allman(fmt) => fmt.list_write_bool(value),
            OutputFormats::Whitesmith(fmt) => fmt.list_write_bool(value),
            OutputFormats::YAML(fmt) => fmt.list_write_bool(value),
        }
    }
    fn list_write_number(&mut self, value: f64) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.list_write_number(value),
            OutputFormats::Linear(fmt) => fmt.list_write_number(value),
            OutputFormats::KNR(fmt) => fmt.list_write_number(value),
            OutputFormats::Allman(fmt) => fmt.list_write_number(value),
            OutputFormats::Whitesmith(fmt) => fmt.list_write_number(value),
            OutputFormats::YAML(fmt) => fmt.list_write_number(value),
        }
    }
    fn list_write_string(&mut self, value: &String) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.list_write_string(value),
            OutputFormats::Linear(fmt) => fmt.list_write_string(value),
            OutputFormats::KNR(fmt) => fmt.list_write_string(value),
            OutputFormats::Allman(fmt) => fmt.list_write_string(value),
            OutputFormats::Whitesmith(fmt) => fmt.list_write_string(value),
            OutputFormats::YAML(fmt) => fmt.list_write_string(value),
        }
    }
    fn list_write_empty_list(&mut self) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.list_write_empty_list(),
            OutputFormats::Linear(fmt) => fmt.list_write_empty_list(),
            OutputFormats::KNR(fmt) => fmt.list_write_empty_list(),
            OutputFormats::Allman(fmt) => fmt.list_write_empty_list(),
            OutputFormats::Whitesmith(fmt) => fmt.list_write_empty_list(),
            OutputFormats::YAML(fmt) => fmt.list_write_empty_list(),
        }
    }
    fn list_write_empty_hash(&mut self) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.list_write_empty_hash(),
            OutputFormats::Linear(fmt) => fmt.list_write_empty_hash(),
            OutputFormats::KNR(fmt) => fmt.list_write_empty_hash(),
            OutputFormats::Allman(fmt) => fmt.list_write_empty_hash(),
            OutputFormats::Whitesmith(fmt) => fmt.list_write_empty_hash(),
            OutputFormats::YAML(fmt) => fmt.list_write_empty_hash(),
        }
    }
    fn list_close(&mut self) -> Result<usize, ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.list_close(),
            OutputFormats::Linear(fmt) => fmt.list_close(),
            OutputFormats::KNR(fmt) => fmt.list_close(),
            OutputFormats::Allman(fmt) => fmt.list_close(),
            OutputFormats::Whitesmith(fmt) => fmt.list_close(),
            OutputFormats::YAML(fmt) => fmt.list_close(),
        }   
    }

    fn hash_open(&mut self) -> Result<usize, ExportError> {
        match self {
            OutputFormats::Compact(fmt) => fmt.hash_open(),
            OutputFormats::Linear(fmt) => fmt.hash_open(),
            OutputFormats::KNR(fmt) => fmt.hash_open(),
            OutputFormats::Allman(fmt) => fmt.hash_open(),
            OutputFormats::Whitesmith(fmt) => fmt.hash_open(),
            OutputFormats::YAML(fmt) => fmt.hash_open(),
        }   
    }
    fn hash_begin_next(&mut self, key: &String) -> Result<(), ExportError> {
        match self {
            OutputFormats::Compact(fmt) => fmt.hash_begin_next(key),
            OutputFormats::Linear(fmt) => fmt.hash_begin_next(key),
            OutputFormats::KNR(fmt) => fmt.hash_begin_next(key),
            OutputFormats::Allman(fmt) => fmt.hash_begin_next(key),
            OutputFormats::Whitesmith(fmt) => fmt.hash_begin_next(key),
            OutputFormats::YAML(fmt) => fmt.hash_begin_next(key),
        }   
    }
    fn hash_write_key(&mut self, key: &String) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.hash_write_key(key),
            OutputFormats::Linear(fmt) => fmt.hash_write_key(key),
            OutputFormats::KNR(fmt) => fmt.hash_write_key(key),
            OutputFormats::Allman(fmt) => fmt.hash_write_key(key),
            OutputFormats::Whitesmith(fmt) => fmt.hash_write_key(key),
            OutputFormats::YAML(fmt) => fmt.hash_write_key(key),
        }   
    }
    fn hash_write_null(&mut self, key: &String) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.hash_write_null(key),
            OutputFormats::Linear(fmt) => fmt.hash_write_null(key),
            OutputFormats::KNR(fmt) => fmt.hash_write_null(key),
            OutputFormats::Allman(fmt) => fmt.hash_write_null(key),
            OutputFormats::Whitesmith(fmt) => fmt.hash_write_null(key),
            OutputFormats::YAML(fmt) => fmt.hash_write_null(key),
        }   
    }
    fn hash_write_bool(&mut self, key: &String, value: bool) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.hash_write_bool(key, value),
            OutputFormats::Linear(fmt) => fmt.hash_write_bool(key, value),
            OutputFormats::KNR(fmt) => fmt.hash_write_bool(key, value),
            OutputFormats::Allman(fmt) => fmt.hash_write_bool(key, value),
            OutputFormats::Whitesmith(fmt) => fmt.hash_write_bool(key, value),
            OutputFormats::YAML(fmt) => fmt.hash_write_bool(key, value),
        }   
    }
    fn hash_write_number(&mut self, key: &String, value: f64) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.hash_write_number(key, value),
            OutputFormats::Linear(fmt) => fmt.hash_write_number(key, value),
            OutputFormats::KNR(fmt) => fmt.hash_write_number(key, value),
            OutputFormats::Allman(fmt) => fmt.hash_write_number(key, value),
            OutputFormats::Whitesmith(fmt) => fmt.hash_write_number(key, value),
            OutputFormats::YAML(fmt) => fmt.hash_write_number(key, value),
        }   
    }
    fn hash_write_string(&mut self, key: &String, value: &String) -> Result<(), ExportError> { 
        match self {
            OutputFormats::Compact(fmt) => fmt.hash_write_string(key, value),
            OutputFormats::Linear(fmt) => fmt.hash_write_string(key, value),
            OutputFormats::KNR(fmt) => fmt.hash_write_string(key, value),
            OutputFormats::Allman(fmt) => fmt.hash_write_string(key, value),
            OutputFormats::Whitesmith(fmt) => fmt.hash_write_string(key, value),
            OutputFormats::YAML(fmt) => fmt.hash_write_string(key, value),
        }   
    }
    fn hash_write_empty_list(&mut self, key: &String) -> Result<(), ExportError> {
        match self {
            OutputFormats::Compact(fmt) => fmt.hash_write_empty_list(key),
            OutputFormats::Linear(fmt) => fmt.hash_write_empty_list(key),
            OutputFormats::KNR(fmt) => fmt.hash_write_empty_list(key),
            OutputFormats::Allman(fmt) => fmt.hash_write_empty_list(key),
            OutputFormats::Whitesmith(fmt) => fmt.hash_write_empty_list(key),
            OutputFormats::YAML(fmt) => fmt.hash_write_empty_list(key),
        }   
    }
    fn hash_write_empty_hash(&mut self, key: &String) -> Result<(), ExportError> {
        match self {
            OutputFormats::Compact(fmt) => fmt.hash_write_empty_hash(key),
            OutputFormats::Linear(fmt) => fmt.hash_write_empty_hash(key),
            OutputFormats::KNR(fmt) => fmt.hash_write_empty_hash(key),
            OutputFormats::Allman(fmt) => fmt.hash_write_empty_hash(key),
            OutputFormats::Whitesmith(fmt) => fmt.hash_write_empty_hash(key),
            OutputFormats::YAML(fmt) => fmt.hash_write_empty_hash(key),
        }   
    }
    fn hash_close(&mut self) -> Result<usize, ExportError> {
        match self {
            OutputFormats::Compact(fmt) => fmt.hash_close(),
            OutputFormats::Linear(fmt) => fmt.hash_close(),
            OutputFormats::KNR(fmt) => fmt.hash_close(),
            OutputFormats::Allman(fmt) => fmt.hash_close(),
            OutputFormats::Whitesmith(fmt) => fmt.hash_close(),
            OutputFormats::YAML(fmt) => fmt.hash_close(),
        }
    }

}

