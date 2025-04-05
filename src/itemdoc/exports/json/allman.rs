use std::result::Result;
use std::io::Write;
use super::super::core::*;
use super::super::core::chardefs::*;
use super::super::core::stringhelp::*;

pub struct JSONFormatAllman {
    writer: Box<dyn Write>,
    context: Box<StructuredExportContext>,
}

impl JSONFormatAllman {
    pub fn new(output: Box<dyn Write>, indent: Option<String>) -> Self {
        Self {
            context: Box::new(StructuredExportContext::new(indent)),
            writer: output,
        }
    }
}

impl ItemOutput for JSONFormatAllman {

    fn list_open(&mut self) -> Result<usize, ExportError> { 
        self.list_begin_next()?;
        let level = self.context.list_begin();
        self.writer.write_all(CHRB_ARR_OPEN_S)?;
        Ok(level)
    }
    fn list_begin_next(&mut self) -> Result<(), ExportError> { 
        if !self.context.is_first() {
            self.writer.write_all(CHRB_COMMA_S)?; 
        }
        Ok(())
    }
    fn list_write_null(&mut self) -> Result<(), ExportError> { 
        self.list_begin_next()?;
        self.writer.write_all(CHRB_NULL)?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_write_bool(&mut self, value: bool) -> Result<(), ExportError> { 
        self.list_begin_next()?;
        self.writer.write_all(value.to_string().as_bytes())?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_write_number(&mut self, value: f64) -> Result<(), ExportError> { 
        self.list_begin_next()?;
        self.writer.write_all(value.to_string().as_bytes())?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_write_string(&mut self, value: &String) -> Result<(), ExportError> { 
        self.list_begin_next()?;
        let escaped = make_quoted_escaped_string(value);
        self.writer.write_all(escaped.as_bytes())?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_write_empty_list(&mut self) -> Result<(), ExportError> { 
        self.list_begin_next()?;
        self.writer.write_all(CHRB_ARR_EMPTY_S)?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_write_empty_hash(&mut self) -> Result<(), ExportError> { 
        self.list_begin_next()?;
        self.writer.write_all(CHRB_OBJ_EMPTY_S)?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_close(&mut self) -> Result<usize, ExportError> { 
        let result = self.context.list_end()?;
        self.writer.write_all(CHRB_ARR_CLOSE_S)?; 
        self.context.incr_item_count();
        Ok(result)
    }

    fn hash_open(&mut self) -> Result<usize, ExportError> {
        let level = self.context.hash_begin();
        self.writer.write_all(CHRB_OBJ_OPEN_S)?;
        Ok(level)
    }
    fn hash_begin_next(&mut self, key: &String) -> Result<(), ExportError> {
        if !self.context.is_first() {
            self.writer.write_all(CHRB_COMMA_S)?; 
        }
        self.hash_write_key(key)?;
        self.writer.write_all(CHRB_COLON_S)?;
        Ok(())
    }
    fn hash_write_key(&mut self, key: &String) -> Result<(), ExportError> { 
        self.hash_begin_next(key)?;
        let enclosed = make_quoted_string(key);
        self.writer.write_all(enclosed.as_bytes())?;
        self.writer.write_all(CHRB_COLON_S)?;
        Ok(())
    }
    fn hash_write_null(&mut self, key: &String) -> Result<(), ExportError> { 
        self.hash_begin_next(key)?;
        self.writer.write_all(CHRB_NULL)?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_write_bool(&mut self, key: &String, value: bool) -> Result<(), ExportError> { 
        self.hash_begin_next(key)?;
        self.writer.write_all(value.to_string().as_bytes())?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_write_number(&mut self, key: &String, value: f64) -> Result<(), ExportError> { 
        self.hash_begin_next(key)?;
        self.writer.write_all(value.to_string().as_bytes())?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_write_string(&mut self, key: &String, value: &String) -> Result<(), ExportError> { 
        self.hash_begin_next(key)?;
        let escaped = make_quoted_escaped_string(value);
        self.writer.write_all(escaped.as_bytes())?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_write_empty_list(&mut self, key: &String) -> Result<(), ExportError> {
        self.hash_begin_next(key)?;
        self.writer.write_all(CHRB_ARR_EMPTY_S)?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_write_empty_hash(&mut self, key: &String) -> Result<(), ExportError> {
        self.hash_begin_next(key)?;
        self.writer.write_all(CHRB_OBJ_EMPTY_S )?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_close(&mut self) -> Result<usize, ExportError> {
        let result = self.context.hash_end()?;
        self.writer.write_all(CHRB_OBJ_CLOSE_S )?;
        self.context.incr_item_count();
        Ok(result)
    }

}
