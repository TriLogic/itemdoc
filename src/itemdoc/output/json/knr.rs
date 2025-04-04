use std::io::Write;
use std::rc::Rc;

use super::super::core::*;
use super::super::core::chardefs::*;
use super::super::core::stringhelp::*;

pub struct JSONFormatKNR {
    writer: Box<dyn Write>,
    context: Box<StructuredOutputContext>,
}

impl JSONFormatKNR {
    pub fn new(output: Box<dyn Write>, indent: Option<String>) -> Self {
        Self {
            context: Box::new(StructuredOutputContext::new(indent)),
            writer: output,
        }
    }

    pub fn get_indent_vec(&self) -> &Rc<[u8]> {
        self.context.get_indent_vec()
    }

    pub fn get_outdent_vec(&self) -> &Rc<[u8]> {
        self.context.get_outdent_vec()
    }

    pub fn write_outdent(&mut self) -> Result<(), OutputError> {
        let outdent = self.get_outdent_vec().clone();
        if outdent.len() > 0 {
            self.writer.write_all(&outdent)?
        }
        Ok(())
    }
    
    pub fn write_indent(&mut self) -> Result<(), OutputError> {
        let indent = self.get_indent_vec().clone();
        if indent.len() > 0 {
            self.writer.write_all(&indent)?;
        }
        Ok(())
    }
}

impl ItemOutput for JSONFormatKNR {

    fn list_open(&mut self) -> Result<usize, OutputError> { 
        if self.context.is_top_level() {
            self.writer.write_all(CHRB_ARR_OPEN_C)?;
        } else {
            if self.context.is_first() {
                self.context.write_indent(self.writer.by_ref())?;
            } else {
                if  self.context.is_within_list() {
                    self.writer.write_all(CHRB_COMMA_S)?;
                }
            }
            let _ =self.writer.write_all(CHRB_ARR_OPEN_C)?;
        }
        Ok(self.context.list_begin())
    }
    fn list_begin_next(&mut self) -> Result<(), OutputError> { 
        if !self.context.is_first() {
            self.writer.write_all(CHRB_COMMA_S)?; 
        }
        self.writer.write_all(CHRB_CRLF)?; 
        self.context.write_indent(self.writer.by_ref())?;
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
        self.writer.write_all(CHRB_ARR_EMPTY_S)?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_write_empty_hash(&mut self) -> Result<(), OutputError> { 
        self.list_begin_next()?;
        self.writer.write_all(CHRB_OBJ_EMPTY_S)?; 
        self.context.incr_item_count();
        Ok(())
    }
    fn list_close(&mut self) -> Result<usize, OutputError>  { 

        let is_top_level = self.context.is_top_level();
        let item_count = self.context.get_item_count();
        let indent_vec = self.context.get_indent_vec().clone();

        let result = self.context.list_end()?;

        if item_count == 0 {
            self.writer.write_all(CHRB_ARR_CLOSE_S)?;
        } else if is_top_level {
            self.writer.write_all(CHRB_ARR_CLOSE_C)?;            
        } else {
            self.writer.write_all(CHRB_CRLF)?;
            self.writer.write_all(&indent_vec)?;
            self.writer.write_all(CHRB_ARR_CLOSE_S)?;    
        }                

        Ok(result)
    }

    fn hash_open(&mut self) -> Result<usize, OutputError> {
        if self.context.is_top_level() {
            self.writer.write_all(CHRB_OBJ_OPEN_C)?;
        } else {
            if self.context.is_first() {
                self.context.write_indent(self.writer.by_ref())?;
            } else {
                if  self.context.is_within_list() {
                    self.writer.write_all(CHRB_COMMA_S)?;
                }
            }
            let _ =self.writer.write_all(CHRB_OBJ_OPEN_C);
        }
        Ok(self.context.hash_begin())
    }
    fn hash_begin_next(&mut self, key: &String) -> Result<(), OutputError> {
        if !self.context.is_first() {
            self.writer.write_all(CHRB_COMMA_S)?; 
        }
        self.hash_write_key(key)?;
        self.writer.write_all(CHRB_COLON_S)?;
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
        let bool_vec = if value { CHRB_TRUE } else {CHRB_FALSE };
        self.writer.write_all(bool_vec )?;
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
        self.writer.write_all(CHRB_ARR_EMPTY_S)?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_write_empty_hash(&mut self, key: &String) -> Result<(), OutputError> {
        self.hash_begin_next(key)?;
        self.writer.write_all(CHRB_OBJ_EMPTY_S )?;
        self.context.incr_item_count();
        Ok(())
    }
    fn hash_close(&mut self) -> Result<usize, OutputError> {
        let result = self.context.hash_end()?;
        self.writer.write_all(CHRB_OBJ_CLOSE_S )?;
        self.context.incr_item_count();
        Ok(result)
    }

}
