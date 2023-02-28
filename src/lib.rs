use wasm_bindgen::prelude::wasm_bindgen;

#[macro_use]
extern crate lazy_static;

pub mod wrappers;
pub mod interpreter;
pub mod parallelism;
mod stdfuncs;

#[wasm_bindgen]
pub fn execute_tin(code: &str) -> String {
    let mut interpreter = interpreter::TinInterpreter::new();
    let parsed_code = interpreter.parse(code);
    
    if let Ok(tokens) = parsed_code {
        let execution = interpreter.execute(&tokens, None, &mut vec!());

        if execution.is_ok() {    
            return interpreter.output;
        
        } else {
            return execution.err().unwrap();
        }
    
    } else {
        return parsed_code.err().unwrap();
    }
}