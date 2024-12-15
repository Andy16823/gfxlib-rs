use std::ffi::CString;

pub fn load_file_as_string(file : String) -> String{
    match std::fs::read_to_string(file) {
        Ok(contents) => {
            return  contents;
        }
        Err(e) => {
            eprint!("Error while loading the file {}", e);
            return String::default();
        }
    }
}

pub fn to_cstr(value : String) -> CString {
   return CString::new(value).expect("CString::new failed");
}