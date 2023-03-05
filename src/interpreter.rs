use std::option::Option;
use std::cmp::*;
use std::collections::HashMap;
use rand::rngs::ThreadRng;
use regex::Regex;

use crate::stdfuncs::std_tin_functions;

#[derive(Clone, Debug, PartialEq)]
pub enum TinValue {
    Int(i64),
    Float(f64),

    Vector(Vec<TinValue>)
}

unsafe impl Send for TinValue {}
unsafe impl Sync for TinValue {}

impl Eq for TinValue {}

impl std::cmp::PartialOrd for TinValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        
        return match (self, other) {
            (TinValue::Vector(_), _) => Option::None,
            (_, TinValue::Vector(_)) => Option::None,

            _ => {
                if crate::wrappers::lt(self, other) == TinValue::Int(1){
                    Option::Some(Ordering::Less)

                } else if self == other {
                    Option::Some(Ordering::Equal)

                } else {
                    Option::Some(Ordering::Greater)
                }
            }   
        }
    }
}

impl std::cmp::Ord for TinValue {
    fn cmp(&self, other: &Self) -> Ordering{
        return self.partial_cmp(other).unwrap();
    }
}

impl TinValue{
    pub fn truthy(&self) -> bool{
        return match self{
            TinValue::Int(n) => *n != 0,
            TinValue::Float(n) => *n != 0.0,
            TinValue::Vector(v) => v.len() != 0,
        };
    }

    pub fn to_string(&self) -> String{
        return match self{
            TinValue::Int(n) => n.to_string(),
            TinValue::Float(n) => format!("{:.5}", n),
            TinValue::Vector(v) => format!("[{}]", v.iter().map(TinValue::to_string).collect::<Vec<_>>().join(", "))
        }
    }
}

#[derive(Clone)]
pub enum TinToken {
    Int(i64),
    Float(f64),

    Fn(String, fn(String, &mut TinInterpreter, &Vec<TinToken>, Option<&Vec<TinToken>>, &mut i64, &mut Vec<TinValue>) -> Result<(), String>),
    Def(String)
}

pub enum TinTokenDetector {
    Regex(Regex),
    Function(fn(&str) -> Option<(TinToken, usize)>)
}

pub struct TinInterpreter {
    pub token_list: Vec<(TinTokenDetector, fn(&str) -> TinToken)>,

    pub variables: HashMap<String, Vec<TinValue>>,
    pub loop_stack: Vec<(i64, Vec<TinValue>, usize)>,
    pub storer_stack: Vec<usize>,
    pub map_stack: Vec<(i64, Vec<TinValue>, usize, usize, Vec<TinValue>)>,
    pub parse_cache: HashMap<String, Vec<TinToken>>,
    pub functions_cache: HashMap<String, Vec<TinToken>>,

    pub output: String,
    pub rng: ThreadRng
}

impl TinInterpreter {
    pub fn new() -> TinInterpreter{
        return TinInterpreter{
            token_list: std_tin_functions(),
            variables: HashMap::new(),
            loop_stack: vec!(),
            storer_stack: vec!(),
            map_stack: vec!(),
            parse_cache: HashMap::new(),
            functions_cache: HashMap::new(),
            output: String::new(),
            rng: rand::thread_rng()
        }
    }

    pub fn parse(&mut self, code_original: &str) -> Result<Vec<TinToken>, String> {
        let mut code = code_original;
        let mut res = vec!();
        
        if self.parse_cache.contains_key(code){
            return Ok(self.parse_cache.get(code).cloned().unwrap());
        }

        while code.len() > 0 {
            let mut opt: Option<(TinToken, usize)> = None;
            code = code.trim_start();

            if code.is_empty() {
                break;
            }

            for (det, f) in &self.token_list {
                match det {
                    TinTokenDetector::Regex(r) => {
                        let m = r.find(code);

                        if m.is_some(){
                            let m_uw = m.unwrap();
                            let m_str = m_uw.as_str();
                            
                            if m_uw.start() == 0 && (opt.is_none() || m_str.len() > opt.as_ref().unwrap().1) {
                                opt = Some((f(m_str), m_str.len()));
                                break;
                            }
                        }
                    }

                    TinTokenDetector::Function(f) => {
                        let m = f(code);

                        if m.is_some(){
                            opt = Some(m.unwrap());
                            break;
                        }
                    }
                }
            }

            if opt.is_none() {
                return Err(format!("Invalid Tin code: [...] {} [...]", code.to_owned()));

            } else {
                let opt_i = opt.unwrap();

                match opt_i.0.clone(){
                    TinToken::Def(s) => {
                        fn exec_func(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>)  -> Result<(), String> {
                            let prg = intrp.functions_cache.get(tok.as_str()).cloned().unwrap();
                            return intrp.execute(&prg, prog_parent, stack);
                        }
    
                        let parts = s.split("|").collect::<Vec<_>>();
                        let func_code = self.parse(parts[1])?;
                        let func_name = parts[3];
    
                        self.functions_cache.entry(func_name.to_string()).or_insert(func_code);
    
                        self.token_list.push((TinTokenDetector::Regex(Regex::new(func_name).unwrap()), |s| TinToken::Fn(s.to_string(), exec_func)));
                    }

                    _ => {}
                }

                res.push(opt_i.0);
                code = &code[opt_i.1..];
            }
        }

        self.parse_cache.entry(code_original.to_string()).or_insert(res.clone());

        return Ok(res);
    }

    pub fn execute(&mut self, program: &Vec<TinToken>, parent: Option<&Vec<TinToken>>, stack: &mut Vec<TinValue>) -> Result<(), String> {
        let mut ip: i64 = 0;

        while ip < program.len() as i64{
            let token = &program[ip as usize];

            match token{
                TinToken::Int(n) => stack.push(TinValue::Int(*n)),
                TinToken::Float(n) => stack.push(TinValue::Float(*n)),
                TinToken::Fn(s, f) => f(s.clone(), self, program, parent, &mut ip, stack).map_err(|err| format!("Error at instruction {} (pos. {}): {}", s, ip, err))?,
                _ => {}
            };

            ip += 1;
        }

        return Ok(());
    }
}