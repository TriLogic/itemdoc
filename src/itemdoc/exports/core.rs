use std::result::Result;
use std::io::Write;
use std::rc::Rc;
use std::fmt;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum ContextError {
    NotAnItemList,
    NotAnItemHash,
}

impl std::error::Error for ContextError {}

#[derive(Debug)]
pub enum ExportError {
    Io(std::io::Error),
    Context(ContextError),
}

impl From<std::io::Error> for ExportError {
    fn from(err: std::io::Error) -> Self {
        ExportError::Io(err)
    }
}

impl From<ContextError> for ExportError {
    fn from(err: ContextError) -> Self {
        ExportError::Context(err)
    }
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextError::NotAnItemList => write!(f, "Not an item list!"),
            ContextError::NotAnItemHash => write!(f, "Not an item hash!"),
        }
    }
}

pub mod chardefs {

    // static values
    pub static CHRB_ARR_OPEN_C: &[u8] = b"[";
    pub static CHRB_ARR_OPEN_S: &[u8] = b"[ ";
    pub static CHRB_ARR_EMPTY_C: &[u8] = b"[]";
    pub static CHRB_ARR_EMPTY_S: &[u8] = b"[ ]";
    pub static CHRB_ARR_CLOSE_C: &[u8] = b"]";
    pub static CHRB_ARR_CLOSE_S: &[u8] = b" ]";

    pub static CHRB_OBJ_OPEN_C: &[u8] = b"{";
    pub static CHRB_OBJ_OPEN_S: &[u8] = b"{ ";
    pub static CHRB_OBJ_EMPTY_C: &[u8] = b"{}";
    pub static CHRB_OBJ_EMPTY_S: &[u8] = b"{ }";
    pub static CHRB_OBJ_CLOSE_C: &[u8] = b"}";
    pub static CHRB_OBJ_CLOSE_S: &[u8] = b" }";

    pub static CHRB_COMMA_C: &[u8] = b",";
    pub static CHRB_COMMA_S: &[u8] = b", ";
    pub static CHRB_COLON_C: &[u8] = b":";
    pub static CHRB_COLON_S: &[u8] = b": ";
    pub static CHRB_DASH_S: &[u8] = b"- ";

    pub static CHRB_CRLF: &[u8] = b"\r\n";
    pub static CHRB_TAB: &[u8] = b"\t";
    pub static CHRB_SPACES: &[u8] = b"  ";
    pub static CHRB_NULL: &[u8] = b"null";
    pub static CHRB_TRUE: &[u8] = b"true";
    pub static CHRB_FALSE: &[u8] = b"false";

}

#[derive(PartialEq)]
pub enum OutputType {
    ListType,
    HashType,
}

pub struct SimpleExportContext {
    stk_type: Vec<OutputType>,
    stk_count: Vec<usize>
}

impl SimpleExportContext {
    pub fn new() -> Self {
        Self {
            stk_type: Vec::new(),
            stk_count: Vec::new()
        }
    }
}

pub struct StructuredExportContext {
    context: SimpleExportContext,
    str_indent: Rc<[u8]>,
    stk_indent: Vec<Rc<[u8]>>,
    vec_empty: Rc<[u8]>,
}

impl StructuredExportContext {
    pub fn new(indent: Option<String>) -> Self {

        let indent_bytes = match indent {
            Some(s) => Rc::from(s.into_bytes()),
            None => Rc::from(chardefs::CHRB_TAB),
        };

        Self {
            context: SimpleExportContext::new(),
            str_indent: indent_bytes,
            stk_indent: Vec::new(),
            vec_empty: Rc::default(),
        }
    }

    fn adjust_indent(&mut self) {
        // if the len of the stk_indent vector is less than the len of the
        // stk_type vector then we need to push a new indent value, if it's
        // the same or larger then we already have value built
        let level = self.context.get_stack_level();

        if self.stk_indent.len() < level {
            match self.stk_indent.len() {
                0 => self.stk_indent.push(self.str_indent.clone()),
                n => { 
                    let new_size = self.str_indent.len() + self.stk_indent[n-1].len();
                    let mut new_indent = Vec::with_capacity(new_size);
                    new_indent.extend_from_slice(&self.str_indent);
                    new_indent.extend_from_slice(&self.stk_indent[n-1]);
                    self.stk_indent.push(Rc::from(new_indent));
                }
            }    
        }
    }

    pub fn get_indent_vec(&self) -> &Rc<[u8]> {
        let level = self.context.get_stack_level();
        let result = match level {
            0 => &self.vec_empty,
            _ => &self.stk_indent[level],
        };
        result
    }

    pub fn get_outdent_vec(&self) -> &Rc<[u8]> {
        let level = self.context.get_stack_level();
        let result = match level {
            0 | 1 => &self.vec_empty,
            _ => &self.stk_indent[level-1]
        };
        result
    }

    pub fn write_outdent(&self, writer: &mut Box<dyn Write, >) -> Result<(), ExportError> {
        let outdent = self.get_outdent_vec().clone();
        if outdent.len() > 0 {
            writer.write(&outdent.clone())?;
        }
        Ok(())
    }
    
    pub fn write_indent(&self, writer: &mut Box<dyn Write>) -> Result<(), ExportError> {
        let indent = self.get_outdent_vec();
        if indent.len() > 0 {
            writer.write_all(indent)?;
        }
        Ok(())
    }

}

pub trait ExportContext {
    fn get_stack_level(&self) -> usize;
    fn get_item_count(&self) -> usize;
    fn incr_item_count(&mut self);

    fn hash_begin(&mut self) -> usize;
    fn hash_end(&mut self) -> Result<usize, ContextError>;
    fn list_begin(&mut self) -> usize;
    fn list_end(&mut self) -> Result<usize, ContextError>;

    fn is_hash(&mut self) -> bool;
    fn is_list(&mut self) -> bool;
    fn is_first(&mut self) -> bool;

    fn is_top_level(&mut self) -> bool;
    fn is_within_list(&mut self) -> bool;
    fn is_within_hash(&mut self) -> bool;
}

impl ExportContext for SimpleExportContext {

    fn get_stack_level(&self) -> usize { self.stk_type.len() }

    fn get_item_count(&self) -> usize {
        match self.stk_count.len() {
            0 => 0,
            n => self.stk_count[n-1],
        }
    }

    fn incr_item_count(&mut self) {
        if self.stk_count.len() > 0 {
            let tos = self.stk_count.len()-1;
            let count = self.stk_count[tos];
            self.stk_count[tos] = count + 1;
        }
    }

    fn hash_begin(&mut self) -> usize {
        self.stk_type.push(OutputType::HashType);
        self.stk_count.push(0);
        self.stk_type.len()
    }

    fn hash_end(&mut self) -> Result<usize, ContextError> {
        self.stk_count.pop();
        let result = match self.stk_type.pop() {
            Some(OutputType::HashType) => Ok(self.stk_type.len()),
            Some(_) | None => Err(ContextError::NotAnItemHash),
        };
        if result.is_ok() {
            self.incr_item_count();
        }
        result
    }

    fn list_begin(&mut self) -> usize {
        self.stk_type.push(OutputType::ListType);
        self.stk_count.push(0);
        self.stk_type.len()
    }

    fn list_end(&mut self) -> Result<usize, ContextError> {
        self.stk_count.pop();
        let result = match self.stk_type.pop() {
            Some(OutputType::ListType) => Ok(self.stk_type.len()),
            Some(_) | None => Err(ContextError::NotAnItemList),
        };
        if result.is_ok() {
            self.incr_item_count();
        }
        result
    }

    fn is_top_level(&mut self) -> bool {
        self.stk_type.len() <= 1
    }
 
    fn is_hash(&mut self) -> bool {
        match self.stk_type.len() {
            0 => false,
            n => self.stk_type[n-1] == OutputType::HashType,
        }
    }

    fn is_list(&mut self) -> bool {
        match self.stk_type.len() {
            0 => false,
            n => self.stk_type[n-1] == OutputType::ListType,
        }
    }

    fn is_first(&mut self) -> bool {
        match self.stk_count.len() {
            0 => true,
            n => self.stk_count[n] == 0,
        }
    }

    fn is_within_hash(&mut self) -> bool {
        match self.stk_type.len() {
            0 | 1 => false,
            n => self.stk_type[n-1] == OutputType::HashType,
        }
    }

    fn is_within_list(&mut self) -> bool {
        match self.stk_type.len() {
            0 | 1 => false,
            n => self.stk_type[n-1] == OutputType::ListType,
        }
    }

}

impl ExportContext for StructuredExportContext {

    fn get_stack_level(&self) -> usize { self.context.get_stack_level() }
    fn get_item_count(&self) -> usize { self.context.get_item_count() }
    fn incr_item_count(&mut self) { self.context.incr_item_count(); }

    fn hash_begin(&mut self) -> usize { 
        self.context.hash_begin();
        self.adjust_indent();
        self.context.get_stack_level()
    }

    fn hash_end(&mut self) -> Result<usize, ContextError> {
        let result = self.context.hash_end();
        self.adjust_indent();
        result
    }

    fn list_begin(&mut self) -> usize {
        self.context.list_begin();
        self.adjust_indent();
        self.context.get_stack_level()
    }

    fn list_end(&mut self) -> Result<usize, ContextError> {
        let result = self.context.list_end();
        self.adjust_indent();
        result
    }

    fn is_top_level(&mut self) -> bool { self.context.is_top_level() }
    fn is_hash(&mut self) -> bool { self.context.is_hash() }
    fn is_list(&mut self) -> bool { self.context.is_list() }
    fn is_first(&mut self) -> bool { self.context.is_first() }
    fn is_within_hash(&mut self) -> bool { self.context.is_within_hash() }
    fn is_within_list(&mut self) -> bool { self.context.is_within_list() }

}

pub trait ItemOutput {
    fn list_open(&mut self) -> Result<usize, ExportError>;
    fn list_begin_next(&mut self) -> Result<(), ExportError>;
    fn list_write_null(&mut self) -> Result<(), ExportError>;
    fn list_write_bool(&mut self, value: bool) -> Result<(), ExportError>;
    fn list_write_number(&mut self, value: f64) -> Result<(), ExportError>;
    fn list_write_string(&mut self, value: &String) -> Result<(), ExportError>;
    fn list_write_empty_list(&mut self) -> Result<(), ExportError>;
    fn list_write_empty_hash(&mut self) -> Result<(), ExportError>;
    fn list_close(&mut self) -> Result<usize, ExportError>;

    fn hash_open(&mut self) -> Result<usize, ExportError>;
    fn hash_begin_next(&mut self, key: &String) -> Result<(), ExportError>;
    fn hash_write_key(&mut self, key: &String) -> Result<(), ExportError>;
    fn hash_write_null(&mut self, key: &String) -> Result<(), ExportError>;
    fn hash_write_bool(&mut self, key: &String, value: bool) -> Result<(), ExportError>;
    fn hash_write_string(&mut self, key: &String, value: &String) -> Result<(), ExportError>;
    fn hash_write_number(&mut self, key: &String, value: f64) -> Result<(), ExportError>;
    fn hash_write_empty_list(&mut self, key: &String) -> Result<(), ExportError>;
    fn hash_write_empty_hash(&mut self, key: &String) -> Result<(), ExportError>;
    fn hash_close(&mut self) -> Result<usize, ExportError>;
}

pub mod stringhelp {

    pub fn make_quoted_string(value: &String) -> String {
        let mut quoted = String::with_capacity(value.len() + 2);
        quoted.push('"');
        quoted.push_str(value);
        quoted.push('"');
        quoted
    }
    
    pub fn make_escaped_string(value: &String) -> String {
        let mut escaped = String::with_capacity(value.len()); // Initial capacity: content + 2 quotes
    
        for c in value.chars() {
            match c {
                '"' => escaped.push_str("\\\""),
                '\\' => escaped.push_str("\\\\"),
                '\n' => escaped.push_str("\\n"),
                '\r' => escaped.push_str("\\r"),
                '\t' => escaped.push_str("\\t"),
                '\u{08}' => escaped.push_str("\\b"), // backspace
                '\u{0C}' => escaped.push_str("\\f"), // formfeed
                c if c < '\u{20}' => {
                    // Control characters (U+0000 to U+001F) in \uXXXX format
                    use std::fmt::Write;
                    write!(escaped, "\\u{:04x}", c as u32).unwrap();
                }
                c => escaped.push(c),
            }
        }
        escaped
    }
    
    pub fn make_quoted_escaped_string(value: &String) -> String {
        let mut escaped = String::with_capacity(value.len() + 2); // Initial capacity: content + 2 quotes
        
        escaped.push('"');
        for c in value.chars() {
            match c {
                '"' => escaped.push_str("\\\""),
                '\\' => escaped.push_str("\\\\"),
                '\n' => escaped.push_str("\\n"),
                '\r' => escaped.push_str("\\r"),
                '\t' => escaped.push_str("\\t"),
                '\u{08}' => escaped.push_str("\\b"), // backspace
                '\u{0C}' => escaped.push_str("\\f"), // formfeed
                c if c < '\u{20}' => {
                    // Control characters (U+0000 to U+001F) in \uXXXX format
                    use std::fmt::Write;
                    write!(escaped, "\\u{:04x}", c as u32).unwrap();
                }
                c => escaped.push(c),
            }
        }
        escaped.push('"');
        escaped
    }

}