use std::result::Result;
use std::io::Write;
use super::super::core::*;
use super::super::core::chardefs::*;
use super::super::core::stringhelp::*;

pub struct JSONFormatCompact {
    writer: Box<dyn Write>,
    context: Box<SimpleOutputContext>,
}

impl JSONFormatCompact {
    pub fn new(output: Box<dyn Write>) -> Self {
        Self {
            context: Box::new(SimpleOutputContext::new()),
            writer: output,
        }
    }
}

impl ItemOutput for JSONFormatCompact {

    fn list_open(&mut self) -> Result<usize, OutputError> { 
        self.list_begin_next()?;
        let level = self.context.list_begin();
        self.writer.write_all(CHRB_ARR_OPEN_C)?;
        Ok(level)
    }
    fn list_begin_next(&mut self) -> Result<(), OutputError> { 
        if !self.context.is_first() {
            self.writer.write_all(CHRB_COMMA_C)?; 
        }
        Ok(())
    }
    fn list_write_null(&mut self) -> Result<(), OutputError> { 
        self.list_begin_next()?;
        self.writer.write_all(CHRB_NULL)?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_write_bool(&mut self, value: bool) -> Result<(), OutputError> { 
        self.list_begin_next()?;
        self.writer.write_all(value.to_string().as_bytes())?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_write_number(&mut self, value: f64) -> Result<(), OutputError> { 
        self.list_begin_next()?;
        self.writer.write_all(value.to_string().as_bytes())?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_write_string(&mut self, value: &String) -> Result<(), OutputError> { 
        self.list_begin_next()?;
        let escaped = make_quoted_escaped_string(value);
        self.writer.write_all(escaped.as_bytes())?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_write_empty_list(&mut self) -> Result<(), OutputError> { 
        self.list_begin_next()?;
        self.writer.write_all(CHRB_ARR_EMPTY_C)?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_write_empty_hash(&mut self) -> Result<(), OutputError> { 
        self.list_begin_next()?;
        self.writer.write_all(CHRB_OBJ_EMPTY_C)?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_close(&mut self) -> Result<usize, OutputError> { 
        let result = self.context.list_end()?;
        self.writer.write_all(CHRB_ARR_CLOSE_C)?; 
        Ok(result)
    }

    fn hash_open(&mut self) -> Result<usize, OutputError> {
        let level = self.context.hash_begin();
        self.writer.write_all(CHRB_OBJ_OPEN_C)?;
        Ok(level)
    }
    fn hash_begin_next(&mut self, key: &String) -> Result<(), OutputError> {
        if !self.context.is_first() {
            self.writer.write_all(CHRB_COMMA_C)?; 
        }
        self.hash_write_key(key)?;
        self.writer.write_all(CHRB_COLON_C)?;
        Ok(())
    }
    fn hash_write_key(&mut self, key: &String) -> Result<(), OutputError> { 
        let enclosed = make_quoted_string(key);
        self.writer.write_all(enclosed.as_bytes())?;
        self.writer.write_all(CHRB_COLON_C)?;
        Ok(())
    }
    fn hash_write_null(&mut self, key: &String) -> Result<(), OutputError> { 
        self.hash_begin_next(key)?;
        self.writer.write_all(CHRB_NULL)?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_write_bool(&mut self, key: &String, value: bool) -> Result<(), OutputError> { 
        self.hash_begin_next(key)?;
        self.writer.write_all(value.to_string().as_bytes())?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_write_number(&mut self, key: &String, value: f64) -> Result<(), OutputError> { 
        self.hash_begin_next(key)?;
        self.writer.write_all(value.to_string().as_bytes())?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_write_string(&mut self, key: &String, value: &String) -> Result<(), OutputError> { 
        self.hash_begin_next(key)?;
        let escaped = make_quoted_escaped_string(value);
        self.writer.write_all(escaped.as_bytes())?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_write_empty_list(&mut self, key: &String) -> Result<(), OutputError> {
        self.hash_begin_next(key)?;
        self.writer.write_all(CHRB_ARR_EMPTY_C)?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_write_empty_hash(&mut self, key: &String) -> Result<(), OutputError> {
        self.hash_begin_next(key)?;
        let _ = self.writer.write_all(CHRB_OBJ_EMPTY_C )?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_close(&mut self) -> Result<usize, OutputError> {
        let result = self.context.hash_end()?;
        let _ = self.writer.write_all(CHRB_OBJ_CLOSE_C )?;
        self.context.incr_item_count();
        Ok(result)
    }

}

