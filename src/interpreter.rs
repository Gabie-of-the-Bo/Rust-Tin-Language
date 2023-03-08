use std::option::Option;
use std::cmp::*;
use std::collections::HashMap;
use rand::rngs::ThreadRng;
use regex::Regex;

use crate::stdfuncs::std_tin_functions;

#[derive(Clone, Debug)]
pub enum TinValue {
    Int(i64),
    Float(f64),

    Vector(Vec<TinValue>),

    // Optimized types
    IntVector(Vec<i64>),
    FloatVector(Vec<f64>)
}

#[derive(Clone, Debug)]
pub enum TinVector {
    Mixed(Vec<TinValue>),
    Int(Vec<i64>),
    Float(Vec<f64>)
}

impl TinVector {
    pub fn get(&self, idx: usize) -> TinValue {
        return match self {
            TinVector::Mixed(v) => v[idx].clone(),
            TinVector::Int(v) => TinValue::Int(v[idx]),
            TinVector::Float(v) => TinValue::Float(v[idx]),
        }
    }

    pub fn len(&self) -> usize {
        return match self {
            TinVector::Mixed(v) => v.len(),
            TinVector::Int(v) => v.len(),
            TinVector::Float(v) => v.len(),
        }
    }

    pub fn push(&mut self, elem: TinValue) {
        return match self {
            TinVector::Mixed(v) => {
                if v.is_empty() {
                    match elem {
                        TinValue::Int(n) => *self = TinVector::Int(vec!(n)), 
                        TinValue::Float(n) => *self = TinVector::Float(vec!(n)), 
                        _ => v.push(elem), 
                    }

                } else {
                    v.push(elem);
                }
            },

            TinVector::Int(v) => {
                match elem {
                    TinValue::Int(n) => v.push(n), 
                    _ => {
                        let mut inner = v.into_iter().map(|i| TinValue::Int(*i)).collect::<Vec<TinValue>>();
                        inner.push(elem);
                        *self = TinVector::Mixed(inner);
                    }, 
                }
            },

            TinVector::Float(v) => {
                match elem {
                    TinValue::Float(n) => v.push(n), 
                    _ => {
                        let mut inner = v.into_iter().map(|i| TinValue::Float(*i)).collect::<Vec<TinValue>>();
                        inner.push(elem);
                        *self = TinVector::Mixed(inner);
                    }, 
                }
            },
        }
    }

    pub fn to_value(self) -> TinValue {
        return match self {
            TinVector::Int(v) => TinValue::IntVector(v),
            TinVector::Float(v) => TinValue::FloatVector(v),
            TinVector::Mixed(v) => TinValue::Vector(v),
        };
    }
}

unsafe impl Send for TinValue {}
unsafe impl Sync for TinValue {}

impl PartialEq for TinValue {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (TinValue::Int(a), TinValue::Int(b)) => a == b,
            (TinValue::Float(a), TinValue::Int(b)) => *a == *b as f64,
            (TinValue::Int(a), TinValue::Float(b)) => *a as f64 == *b,
            (TinValue::Float(a), TinValue::Float(b)) => a == b,

            (TinValue::Vector(a), TinValue::Vector(b)) => a.len() == b.len() && a.iter().zip(b).all(|(i, j)| i == j),
            (TinValue::Vector(a), TinValue::IntVector(b)) => a.len() == b.len() && a.iter().zip(b).all(|(i, j)| *i == TinValue::Int(*j)),
            (TinValue::IntVector(a), TinValue::Vector(b)) => a.len() == b.len() && a.iter().zip(b).all(|(i, j)| TinValue::Int(*i) == *j),
            (TinValue::Vector(a), TinValue::FloatVector(b)) => a.len() == b.len() && a.iter().zip(b).all(|(i, j)| *i == TinValue::Float(*j)),
            (TinValue::FloatVector(a), TinValue::Vector(b)) => a.len() == b.len() && a.iter().zip(b).all(|(i, j)| TinValue::Float(*i) == *j),
            (TinValue::FloatVector(a), TinValue::IntVector(b)) => a.len() == b.len() && a.iter().zip(b).all(|(i, j)| *i == *j as f64),
            (TinValue::IntVector(a), TinValue::FloatVector(b)) => a.len() == b.len() && a.iter().zip(b).all(|(i, j)| *i as f64 == *j),
            (TinValue::IntVector(a), TinValue::IntVector(b)) => a.len() == b.len() && a.iter().zip(b).all(|(i, j)| *i == *j),
            (TinValue::FloatVector(a), TinValue::FloatVector(b)) => a.len() == b.len() && a.iter().zip(b).all(|(i, j)| *i == *j),

            _ => false
        }
    }
}

impl Eq for TinValue {}

impl std::cmp::PartialOrd for TinValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        
        return match (self, other) {
            (TinValue::Int(a), TinValue::Int(b)) => Some(a.cmp(b)),
            (TinValue::Float(a), TinValue::Int(b)) => a.partial_cmp(&(*b as f64)),
            (TinValue::Int(a), TinValue::Float(b)) => (*a as f64).partial_cmp(b),
            (TinValue::Float(a), TinValue::Float(b)) => a.partial_cmp(b),

            (TinValue::Vector(a), TinValue::Vector(b)) => {
                let res = a.len().cmp(&b.len());

                for c in a.iter().zip(b.iter()).map(|(i, j)| i.cmp(j)) {
                    if c.is_ne() {
                        return Some(c);
                    }
                }

                Some(res)
            }

            (TinValue::Vector(a), TinValue::IntVector(b)) => {
                let res = a.len().cmp(&b.len());

                for c in a.iter().zip(b.iter()).map(|(i, j)| i.cmp(&TinValue::Int(*j))) {
                    if c.is_ne() {
                        return Some(c);
                    }
                }

                Some(res)
            }

            (TinValue::Vector(a), TinValue::FloatVector(b)) => {
                let res = a.len().cmp(&b.len());

                for c in a.iter().zip(b.iter()).map(|(i, j)| i.cmp(&TinValue::Float(*j))) {
                    if c.is_ne() {
                        return Some(c);
                    }
                }

                Some(res)
            }
            
            (TinValue::IntVector(a), TinValue::IntVector(b)) => {
                let res = a.len().cmp(&b.len());

                for c in a.iter().zip(b.iter()).map(|(i, j)| i.cmp(j)) {
                    if c.is_ne() {
                        return Some(c);
                    }
                }

                Some(res)
            }
            
            (TinValue::IntVector(a), TinValue::Vector(b)) => {
                let res = a.len().cmp(&b.len());

                for c in a.iter().zip(b.iter()).map(|(i, j)| TinValue::Int(*i).cmp(j)) {
                    if c.is_ne() {
                        return Some(c);
                    }
                }

                Some(res)
            }
            
            (TinValue::IntVector(a), TinValue::FloatVector(b)) => {
                let res = a.len().cmp(&b.len());

                for c in a.iter().zip(b.iter()).map(|(i, j)| TinValue::Int(*i).cmp(&TinValue::Float(*j))) {
                    if c.is_ne() {
                        return Some(c);
                    }
                }

                Some(res)
            }
            
            (TinValue::FloatVector(a), TinValue::FloatVector(b)) => {
                let res = a.len().cmp(&b.len());

                for c in a.iter().zip(b.iter()).map(|(i, j)| i.partial_cmp(j).unwrap()) {
                    if c.is_ne() {
                        return Some(c);
                    }
                }

                Some(res)
            }
            
            (TinValue::FloatVector(a), TinValue::IntVector(b)) => {
                let res = a.len().cmp(&b.len());

                for c in a.iter().zip(b.iter()).map(|(i, j)| TinValue::Float(*i).cmp(&TinValue::Int(*j))) {
                    if c.is_ne() {
                        return Some(c);
                    }
                }

                Some(res)
            }
            
            (TinValue::FloatVector(a), TinValue::Vector(b)) => {
                let res = a.len().cmp(&b.len());

                for c in a.iter().zip(b.iter()).map(|(i, j)| TinValue::Float(*i).cmp(j)) {
                    if c.is_ne() {
                        return Some(c);
                    }
                }

                Some(res)
            }

            (TinValue::Vector(_), _) |
            (TinValue::IntVector(_), _) |
            (TinValue::FloatVector(_), _) => Some(Ordering::Greater),

            (_, TinValue::Vector(_)) |
            (_, TinValue::IntVector(_)) |
            (_, TinValue::FloatVector(_)) => Some(Ordering::Less),
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
            TinValue::IntVector(v) => v.len() != 0,
            TinValue::FloatVector(v) => v.len() != 0,
        };
    }

    pub fn to_string(&self) -> String{
        return match self{
            TinValue::Int(n) => n.to_string(),
            TinValue::Float(n) => format!("{:.5}", n),
            TinValue::Vector(v) => format!("[{}]", v.iter().map(TinValue::to_string).collect::<Vec<_>>().join(", ")),
            TinValue::IntVector(v) => format!("[{}]", v.iter().map(i64::to_string).collect::<Vec<_>>().join(", ")),
            TinValue::FloatVector(v) => format!("[{}]", v.iter().map(f64::to_string).collect::<Vec<_>>().join(", "))
        }
    }

    pub fn to_mapped_string(&self) -> Result<String, String> {
        return match self {
            TinValue::Int(n) => char::from_u32(*n as u32).map(|i| i.to_string()).ok_or(format!("Unable to convert {} to string", n)),
            TinValue::Float(n) => return Err(format!("Unable to convert {} to string", n)),
            TinValue::Vector(v) => Ok(v.iter().map(TinValue::to_mapped_string).collect::<Result<Vec<_>, String>>()?.join("")),
            TinValue::IntVector(v) => Ok(v.iter().map(|i| TinValue::Int(*i)).map(|i| i.to_mapped_string()).collect::<Result<Vec<_>, String>>()?.join("")),
            TinValue::FloatVector(_) => return Err(format!("Unable to convert float vector to string")),
        }
    }
}

#[derive(Clone)]
pub enum TinToken {
    Int(i64),
    Float(f64),
    String(String),

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
    pub loop_stack: Vec<(i64, TinVector, usize)>,
    pub storer_stack: Vec<usize>,
    pub map_stack: Vec<(i64, TinVector, usize, usize, TinVector)>,
    pub while_stack: Vec<i64>,
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
            while_stack: vec!(),
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

            // Comments
            if code.starts_with("∴") {
                code = &code[3..];

                let next_dots = code.find('∴').map(|i| i + 3);
                let next_line = code.find('\n').map(|i| i + 1);
                
                let pos = if next_dots.is_some() || next_line.is_some() {
                    next_dots.unwrap_or(usize::MAX).min(next_line.unwrap_or(usize::MAX))
                
                } else {
                    code.len()
                };

                code = &code[pos..].trim_start();
            }

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
                TinToken::String(s) => stack.push(TinValue::IntVector(s.chars().map(|i| i as i64).collect())),
                TinToken::Fn(s, f) => f(s.clone(), self, program, parent, &mut ip, stack).map_err(|err| format!("Error at instruction {} (pos. {}): {}", s, ip, err))?,
                _ => {}
            };

            ip += 1;
        }

        return Ok(());
    }
}