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

    pub fn to_string(&self) -> String{
        return match self{
            TinValue::INT(n) => n.to_string(),
            TinValue::FLOAT(n) => format!("{:.5}", n),
            TinValue::VECTOR(v) => format!("[{}]", v.iter().map(TinValue::to_string).collect::<Vec<_>>().join(", ")),
            TinValue::NONE => "NONE".to_string()
        }
    }
}

#[derive(Clone)]
pub enum TinToken {
    INT(i64),
    FLOAT(f64),

    FN(String, fn(String, &mut TinInterpreter, &Vec<TinToken>, &mut usize, &mut Vec<TinValue>) -> TinValue),
    DEF(String)
}

pub struct TinInterpreter {
    pub token_list: Vec<(Regex, fn(&str) -> TinToken)>,

    pub variables: HashMap<String, Vec<TinValue>>,
    pub loop_stack: Vec<(usize, Vec<TinValue>, usize)>,
    pub storer_stack: Vec<usize>,
    pub functions_cache: HashMap<String, Vec<TinToken>>
}

impl TinInterpreter {
    pub fn new() -> TinInterpreter{
        return TinInterpreter{
            token_list: std_tin_functions(),
            variables: HashMap::new(),
            loop_stack: vec!(),
            storer_stack: vec!(),
            functions_cache: HashMap::new()
        }
    }

    pub fn parse(&mut self, code_original: &str) -> Vec<TinToken>{
        let mut code = code_original;
        let mut res = vec!();
        
        while code.len() > 0 {
            let mut opt: Option<(TinToken, usize)> = None;

            if code.starts_with(" "){
                code = &code[1..];
                continue;
            }

            for (r, f) in &self.token_list {
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

                if let TinToken::DEF(s) = opt_i.0.clone() {
                    fn exec_func(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
                        let prg = intrp.functions_cache.get(tok.as_str()).cloned().unwrap();
                        intrp.execute(&prg, stack);

                        return TinValue::NONE;
                    }

                    let parts = s.split("|").collect::<Vec<_>>();
                    let func_code = self.parse(parts[1]);
                    let func_name = parts[3];

                    self.functions_cache.entry(func_name.to_string()).or_insert(func_code);

                    self.token_list.push((Regex::new(func_name).unwrap(), |s| TinToken::FN(s.to_string(), exec_func)));
                }

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

                _ => {}
            };

            ip += 1;
        }
    }
}