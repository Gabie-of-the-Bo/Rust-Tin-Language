use std::collections::HashMap;
use regex::Regex;

use crate::stdfuncs::std_tin_functions;

#[derive(Clone, Debug, PartialEq)]
pub enum TinValue {    
    NONE,

    INT(i64),
    FLOAT(f64),

    VECTOR(Vec<TinValue>)
}

impl TinValue{
    pub fn truthy(&self) -> bool{
        return match self{
            TinValue::INT(n) => *n != 0,
            TinValue::FLOAT(n) => *n != 0.0,
            TinValue::VECTOR(v) => v.len() != 0,

            _ => false
        };
    }
}

pub enum TinToken {
    INT(i64),
    FLOAT(f64),

    FN(String, fn(String, &mut TinInterpreter, &Vec<TinToken>, &mut usize, &mut Vec<TinValue>) -> TinValue)
}

pub struct TinInterpreter {
    pub token_list: Vec<(Regex, fn(&str) -> TinToken)>,

    pub variables: HashMap<String, Vec<TinValue>>,
    pub loop_stack: Vec<(usize, Vec<TinValue>, usize)>,
    pub storer_stack: Vec<usize>
}

impl TinInterpreter {
    pub fn new() -> TinInterpreter{
        return TinInterpreter{
            token_list: std_tin_functions(),
            variables: HashMap::new(),
            loop_stack: vec!(),
            storer_stack: vec!()
        }
    }

    pub fn parse(&self, code_original: &str) -> Vec<TinToken>{
        let mut code = code_original;
        let mut res = vec!();
        
        while code.len() > 0 {
            let mut opt: Option<(TinToken, usize)> = None;

            if code.starts_with(" "){
                code = &code[1..];
                continue;
            }

            for (r, f) in &self.token_list{
                let m = r.find(code);

                if m.is_some(){
                    let m_uw = m.unwrap();
                    let m_str = m_uw.as_str();
                    
                    if m_uw.start() == 0 && (opt.is_none() || m_str.len() > opt.as_ref().unwrap().1) {
                        opt = Some((f(m_str), m_str.len()));
                    }
                }
            }

            if opt.is_none() {
                panic!(format!("Invalid Tin code: [...] {} [...]", code.to_owned()));

            } else {
                let opt_i = opt.unwrap();

                res.push(opt_i.0);
                code = &code[opt_i.1..];
            }
        }

        return res;
    }

    pub fn execute(&mut self, program: &Vec<TinToken>, stack: &mut Vec<TinValue>){
        let mut ip = 0;

        while ip < program.len(){
            let token = &program[ip];

            match token{
                TinToken::INT(n) => stack.push(TinValue::INT(*n)),
                TinToken::FLOAT(n) => stack.push(TinValue::FLOAT(*n)),

                TinToken::FN(s, f) => {
                    let res = f(s.to_string(), self, program, &mut ip, stack);
                    
                    if res != TinValue::NONE {
                        stack.push(res);
                    }
                }
            };

            ip += 1;
        }
    }
}