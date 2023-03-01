use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

use crate::wrappers;
use crate::interpreter::{*};
use crate::parallelism;

fn safe_pop(stack: &mut Vec<TinValue>) -> Result<TinValue, String> {
    return match stack.pop() {
        Some(obj) => Ok(obj),
        _ => Err("Unable to pop from empty stack".into())
    };
}

fn safe_peek(stack: &mut Vec<TinValue>) -> Result<&mut TinValue, String> {
    return match stack.last_mut() {
        Some(obj) => Ok(obj),
        _ => Err("Unable to peek from empty stack".into())
    };
}

fn tin_drop(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    safe_pop(stack)?;
        
    return Ok(());
}

fn tin_dup(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let elem = safe_peek(stack)?.clone();
    stack.push(elem);
    
    return Ok(());
}

fn tin_swap(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let last_index = stack.len() - 1;
    stack.swap(last_index, last_index - 1);
    
    return Ok(());
}

fn tin_copy(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    if let TinValue::Int(n) = safe_pop(stack)? {
        let item = stack.get(stack.len() - 1 - n as usize).cloned().ok_or(format!("Unable to pop {}th element from stack of size {}", n, stack.len()))?;
        stack.push(item);
    
    } else {
        return Err("Popped element was not an int".into());
    }
    
    return Ok(());
}

fn tin_define_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let ctx = intrp.variables.entry(tok).or_insert(vec!());
    ctx.push(safe_pop(stack)?);
    
    return Ok(());
}

fn tin_redefine_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let ctx = intrp.variables.entry(tok).or_insert(vec!());
    
    if ctx.len() > 0 {
        *ctx.last_mut().unwrap() = safe_pop(stack)?;

    } else {
        ctx.push(safe_pop(stack)?);
    }
    
    return Ok(());
}

fn tin_delete_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, _stack: &mut Vec<TinValue>) -> Result<(), String> {
    if intrp.variables.contains_key(&tok.to_string()) {
        let ctx = intrp.variables.get_mut(&tok).unwrap();
        ctx.pop();

        if ctx.len() == 0 {
            intrp.variables.remove(&tok);
        }
    }
    
    return Ok(());
}

fn tin_get_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let ctx = &intrp.variables[&tok];
    stack.push(ctx.last().cloned().ok_or(format!("Variable \'{}\' is not defined", tok))?);
    
    return Ok(());
}

fn tin_skip(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let last_t = safe_pop(stack)?.truthy();

    if !last_t{
        *ip += 1;
    }
    
    return Ok(());
}

fn tin_skip_dup(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let last_t = stack.last().as_ref().unwrap().truthy();

    if !last_t{
        *ip += 1;
    }
    
    return Ok(());
}

fn tin_skip_inv(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let last_t = safe_pop(stack)?.truthy();

    if last_t{
        *ip += 1;
    }
    
    return Ok(());
}

fn tin_foreach_init(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    if !intrp.loop_stack.is_empty() && intrp.loop_stack.last().unwrap().0 == *ip{
        intrp.loop_stack.last_mut().unwrap().2 += 1;

    } else{
        match safe_pop(stack)? {
            TinValue::Vector(v) => intrp.loop_stack.push((*ip, v, 0)),

            _ => return Err("Popped element was not a vector".into())
        }
    }

    stack.push(intrp.loop_stack.last().unwrap().1[intrp.loop_stack.last().unwrap().2].clone());
    
    return Ok(());
}

fn tin_foreach_end(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, _stack: &mut Vec<TinValue>) -> Result<(), String> {
    let (pos, arr, idx) = intrp.loop_stack.last().unwrap();

    if *idx < arr.len() - 1 {
        *ip = pos - 1;

    } else{
        intrp.loop_stack.pop();
    }
    
    return Ok(());
}

fn tin_storer_init(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    intrp.storer_stack.push(stack.len());
    
    return Ok(());
}

fn tin_storer_end(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let idx = intrp.storer_stack.pop().unwrap();
    
    let arr = TinValue::Vector(stack.drain(idx..).collect::<Vec<_>>());
    stack.push(arr);
    
    return Ok(());
}

fn tin_map_init(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    if !intrp.map_stack.is_empty() && intrp.map_stack.last().unwrap().0 == *ip{
        intrp.map_stack.last_mut().unwrap().2 += 1;

    } else{
        match safe_pop(stack)? {
            TinValue::Vector(v) => intrp.map_stack.push((*ip, v, 0, stack.len(), vec!())),

            _ => return Err("Popped element was not a vector".into())
        }
    }

    stack.push(intrp.map_stack.last().unwrap().1[intrp.map_stack.last().unwrap().2].clone());
    
    return Ok(());
}

fn tin_map_end(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let (pos, arr, idx, stack_pos, acum) = intrp.map_stack.last_mut().unwrap();

    acum.append(&mut stack.drain(*stack_pos..).collect());

    if *idx < arr.len() - 1 {
        *ip = *pos - 1;

    } else{
        let (_, _, _, _, acum) = intrp.map_stack.pop().unwrap();
        stack.push(TinValue::Vector(acum));
    }
    
    return Ok(());
}

fn nabla(_tok: String, intrp: &mut TinInterpreter, prog: &Vec<TinToken>, prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    if prog_parent.is_some(){
        intrp.execute(prog_parent.unwrap(), Option::None, stack)?;

    } else{
        intrp.execute(prog, prog_parent, stack)?;
    }
    
    return Ok(());
}

fn block(tok: String, intrp: &mut TinInterpreter, prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let new_tok = &tok[3..tok.len() - 3];
    let program = intrp.parse(new_tok)?;
    intrp.execute(&program, Option::Some(prog), stack)?;
    
    return Ok(());
}

fn rec_block(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let new_tok = &tok[3..tok.len() - 3];
    let program = intrp.parse(new_tok)?;
    intrp.execute(&program, Option::Some(&program), stack)?;
    
    return Ok(());
}

fn tin_eq(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_pop(stack)?;
    let b = safe_peek(stack)?;

    *stack.last_mut().unwrap() = TinValue::Int((a == *b) as i64);
    
    return Ok(());
}

fn tin_lt(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_pop(stack)?;
    let b = safe_peek(stack)?;

    *stack.last_mut().unwrap() = wrappers::lt(&a, &b);
    
    return Ok(());
}

fn tin_gt(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_pop(stack)?;
    let b = safe_peek(stack)?;

    *stack.last_mut().unwrap() = wrappers::gt(&a, &b);
    
    return Ok(());
}

fn tin_sum(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_pop(stack)?;
    let b = safe_peek(stack)?;

    *stack.last_mut().unwrap() = wrappers::sum(&a, &b);
    
    return Ok(());
}

fn tin_sub(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_pop(stack)?;
    let b = safe_peek(stack)?;

    *stack.last_mut().unwrap() = wrappers::sub(&a, &b);
    
    return Ok(());
}

fn tin_mul(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_pop(stack)?;
    let b = safe_peek(stack)?;

    *stack.last_mut().unwrap() = wrappers::mul(&a, &b);
    
    return Ok(());
}

fn tin_div(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_pop(stack)?;
    let b = safe_peek(stack)?;

    *stack.last_mut().unwrap() = wrappers::div(&a, &b);
    
    return Ok(());
}

fn tin_mod(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_pop(stack)?;
    let b = safe_peek(stack)?;

    *stack.last_mut().unwrap() = wrappers::modl(&a, &b);
    
    return Ok(());
}

fn tin_pow(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_pop(stack)?;
    let b = safe_peek(stack)?;

    *stack.last_mut().unwrap() = wrappers::pow(&a, &b);
    
    return Ok(());
}

fn tin_sqrt(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_peek(stack)?;

    *stack.last_mut().unwrap() = wrappers::sqrt(&a);
    
    return Ok(());
}

fn tin_inc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let one = TinValue::Int(1);

    *stack.last_mut().unwrap() = wrappers::sum(&stack.last().unwrap(), &one);
    
    return Ok(());
}

fn tin_dec(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let one = TinValue::Int(1);

    *stack.last_mut().unwrap() = wrappers::sub(&stack.last().unwrap(), &one);
    
    return Ok(());
}

fn tin_floor(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    *stack.last_mut().unwrap() = wrappers::floor(&stack.last().unwrap());
    
    return Ok(());
}

fn tin_ceil(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    *stack.last_mut().unwrap() = wrappers::ceil(&stack.last().unwrap());

    
    return Ok(());
}

fn tin_truthy(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    *stack.last_mut().unwrap() = wrappers::truthy(&stack.last().unwrap());
    
    return Ok(());
}

fn tin_neg(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    *stack.last_mut().unwrap() = wrappers::neg(&stack.last().unwrap());
    
    return Ok(());
}

fn tin_or(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_pop(stack)?;
    let b = safe_peek(stack)?;

    *stack.last_mut().unwrap() = wrappers::or(&a, &b);
    
    return Ok(());
}

fn tin_and(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let a = safe_pop(stack)?;
    let b = safe_peek(stack)?;

    *stack.last_mut().unwrap() = wrappers::and(&a, &b);
    
    return Ok(());
}

fn tin_any(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let res = match safe_pop(stack)? {
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_any(v)

            } else{
                TinValue::Int(v.iter().any(TinValue::truthy) as i64)
            }
        },

        _ => return Err("Popped element was not a vector".into())
    };
    
    stack.push(res);
    
    return Ok(());
}

fn tin_none(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let res = match safe_pop(stack)? {
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_none(v)

            } else{
                TinValue::Int(!v.iter().any(TinValue::truthy) as i64)
            }
        },

        _ => return Err("Popped element was not a vector".into())
    };
    
    stack.push(res);
    
    return Ok(());
}

fn tin_all(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let res = match safe_pop(stack)? {
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_all(v)

            } else{
                TinValue::Int(v.iter().all(TinValue::truthy) as i64)
            }
        },
        
        _ => return Err("Popped element was not a vector".into())
    };
    
    stack.push(res);
    
    return Ok(());
}

fn iota(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let res = match safe_pop(stack)? {
        TinValue::Int(a) => TinValue::Vector((0..a).map(TinValue::Int).collect::<Vec<_>>()),
        TinValue::Float(a) => TinValue::Vector((0..a as i64).map(TinValue::Int).collect::<Vec<_>>()),

        _ => return Err("Popped element was not an int or a float".into())
    };
    
    stack.push(res);
    
    return Ok(());
}

fn boxed(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let res = TinValue::Vector(vec!(safe_pop(stack)?));
    stack.push(res);
    
    return Ok(());
}

fn set(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let idx = safe_pop(stack)?;
    let elem = safe_pop(stack)?;
    let v = safe_peek(stack)?;

    match (idx, v) {
        (TinValue::Int(a), TinValue::Vector(v)) => v[a as usize] = elem,

        _ => return Err("Popped elements were not an int and a vector".into())
    };
    
    return Ok(());
}

fn get(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let idx = safe_pop(stack)?;
    let v = safe_pop(stack)?;

    let res = match (idx, v) {
        (TinValue::Int(a), TinValue::Vector(v)) => v[a as usize].clone(),

        _ => return Err("Popped elements were not an int and a vector".into())
    };
    
    stack.push(res);
    
    return Ok(());
}

fn get_nc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let idx = safe_pop(stack)?;
    let v = safe_peek(stack)?;

    let res = match (idx, v) {
        (TinValue::Int(a), TinValue::Vector(v)) => v[a as usize].clone(),

        _ => return Err("Popped elements were not an int and a vector".into())
    };
    
    stack.push(res);
    
    return Ok(());
}

fn tin_sum_all(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    match safe_pop(stack)? {
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                stack.push(parallelism::parallel_sum_all(v));

            } else{
                let mut res = TinValue::Int(0);

                for i in v{
                    res = wrappers::sum(&res, &i);
                }
    
                stack.push(res);
            }
        },

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_mul_all(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    match safe_pop(stack)? {
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                stack.push(parallelism::parallel_mul_all(v));

            } else{
                let mut res = TinValue::Int(1);

                for i in v{
                    res = wrappers::mul(&res, &i);
                }
    
                stack.push(res);
            }
        },

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_len(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let res = match safe_pop(stack)? {
        TinValue::Vector(v) => TinValue::Int(v.len() as i64),

        _ => return Err("Popped element was not a vector".into())
    };
    
    stack.push(res);
    
    return Ok(());
}

fn tin_max(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    match safe_pop(stack)? {
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()) {
                stack.push(parallelism::parallel_max(v));

            } else{
                let mut v_it = v.iter();
                let mut res = v_it.next().unwrap();
    
                for i in v_it{
                    if let TinValue::Int(1) = wrappers::gt(&i, &res){
                        res = i;
                    }
                }
    
                stack.push(res.clone());
            }
        }

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_min(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    match safe_pop(stack)? {
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()) {
                stack.push(parallelism::parallel_min(v));

            } else{
                let mut v_it = v.iter();
                let mut res = v_it.next().unwrap();
    
                for i in v_it{
                    if let TinValue::Int(1) = wrappers::lt(&i, &res){
                        res = i;
                    }
                }
    
                stack.push(res.clone());
            }
        }

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_count(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let elem = safe_pop(stack)?;
    
    match safe_pop(stack)? {
        TinValue::Vector(v) => {
            let mut res = 0;

            for i in v{
                if i == elem {
                    res += 1;
                }
            }

            stack.push(TinValue::Int(res));
        }

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_nc_count(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let elem = safe_pop(stack)?;
    
    match safe_peek(stack)? {
        TinValue::Vector(v) => {
            let mut res = 0;

            for i in v{
                if *i == elem {
                    res += 1;
                }
            }

            stack.push(TinValue::Int(res));
        }

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_index(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let elem = safe_pop(stack)?;
    
    match safe_pop(stack)? {
        TinValue::Vector(v) => {
            let mut res = vec!();

            for (idx, i) in v.iter().enumerate(){
                if *i == elem {
                    res.push(TinValue::Int(idx as i64));
                }
            }

            stack.push(TinValue::Vector(res));
        }

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_nc_index(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let elem = safe_pop(stack)?;
    
    match safe_peek(stack)? {
        TinValue::Vector(v) => {
            let mut res = vec!();

            for (idx, i) in v.iter().enumerate(){
                if *i == elem {
                    res.push(TinValue::Int(idx as i64));
                }
            }

            stack.push(TinValue::Vector(res));
        }

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_from_index(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    let mut idx_vec = safe_pop(stack)?;
    let mut ref_vec = safe_peek(stack)?;
    
    match (&mut ref_vec, &mut idx_vec) {
        (TinValue::Vector(ref_v), TinValue::Vector(idx)) => {
            *ref_v = idx.iter().map(|i| match i {
                TinValue::Int(n) => Ok(ref_v[*n as usize].clone()),
                _ => Err("Popped element was not an int".to_string())
            }).collect::<Result<Vec<TinValue>, String>>()?;
        }

        _ => return Err("Popped elements were not two vectors".into())
    };
    
    return Ok(());
}

fn tin_sort_asc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {    
    match safe_peek(stack)? {
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_sort_asc(v);
                
            } else{
                v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            }
        }

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_sort_idx_asc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {    
    match safe_peek(stack)? {
        TinValue::Vector(v) => {
            let mut v_cpy = v.iter().enumerate().collect::<Vec<_>>();
            
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_sort_idx_asc(&mut v_cpy);

            } else{
                v_cpy.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap()); 
            }

            *v = v_cpy.iter().map(|t| TinValue::Int(t.0 as i64)).collect();
        }

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_sort_desc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {    
    match safe_peek(stack)? {
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_sort_desc(v);
                
            } else{
                v.sort_by(|a, b| b.partial_cmp(a).unwrap()); 
            }
        }

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_sort_idx_desc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {    
    match safe_peek(stack)? {
        TinValue::Vector(v) => {
            let mut v_cpy = v.iter().enumerate().collect::<Vec<_>>();
            
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_sort_idx_desc(&mut v_cpy);

            } else{
                v_cpy.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap()); 
            }

            *v = v_cpy.iter().map(|t| TinValue::Int(t.0 as i64)).collect();
        }

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_unique(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {    
    match safe_peek(stack)? {
        TinValue::Vector(v) => {
            *v = v.iter().collect::<BTreeSet<_>>().iter().map(|i| (*i).clone()).collect();
        },

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_counts(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {    
    match safe_peek(stack)? {
        TinValue::Vector(v) => {
            let mut counts = BTreeMap::new();

            for i in v.iter() {
                counts.entry(i).and_modify(|e| *e += 1).or_insert(1);
            }

            *v = v.iter().map(|i| TinValue::Int(*counts.get(i).unwrap() as i64)).collect();
        },

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_merge(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {    
    let mut v1 = safe_pop(stack)?;
    let mut v2 = safe_peek(stack)?;

    match (&mut v1, &mut v2) {
        (TinValue::Vector(a), TinValue::Vector(b)) => b.append(a),

        _ => return Err("Popped elements were not two vectors".into())
    };
    
    return Ok(());
}

fn tin_append(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {    
    let elem = safe_pop(stack)?;

    match safe_peek(stack)? {
        TinValue::Vector(v) => v.push(elem),

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn drop_first(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    match safe_peek(stack)? {
        TinValue::Vector(v) => {
            if v.len() > 0 {
                v.remove(0);
            }
        },

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn drop_last(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    match safe_peek(stack)? {
        TinValue::Vector(v) => {
            if v.len() > 0 {
                v.pop();
            }
        },

        _ => return Err("Popped element was not a vector".into())
    };
    
    return Ok(());
}

fn tin_print(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>) -> Result<(), String> {
    if cfg!(target_arch = "wasm32") {
        intrp.output += &safe_pop(stack)?.to_string();

    } else {
        println!("{}", safe_pop(stack)?.to_string());
    }
    
    return Ok(());
}

macro_rules! block_parse_funct {
    ($name: ident, $execution: ident, $open: literal, $close: literal) => {
        fn $name(code: &str) -> Option<(TinToken, usize)> {
            let mut depth = 0;
            let mut i = 0;
        
            for char in code.chars() {
                match char {
                    $open => depth += 1,
                    $close => depth -= 1,
                    _ => {}
                }
        
                match (i, depth) {
                    (0, 0) | (_, -1) => return None,
                    (_, 0) => {
                        i += char.len_utf8();
                        return Some((TinToken::Fn(code[..i].to_string(), $execution), i))
                    },
                    _ => i += char.len_utf8()
                }
            }
        
            return None;
        }
    };
}

block_parse_funct!(parse_block, block, '⟨', '⟩');
block_parse_funct!(parse_rec_block, rec_block, '⦑', '⦒');

pub fn std_tin_functions() -> Vec<(TinTokenDetector, fn(&str) -> TinToken)> {
    let from_re = |s| TinTokenDetector::Regex(Regex::new(s).unwrap());
    let from_fn = |f| TinTokenDetector::Function(f);
    
    let res: Vec<(TinTokenDetector, fn(&str) -> TinToken)> = vec!(
        // Literals
        (from_re(r"\d*\.\d+"), |s| TinToken::Float(s.parse::<f64>().unwrap())),
        (from_re(r"\d+"), |s| TinToken::Int(s.parse::<i64>().unwrap())),

        // Meta
        (from_re(r"¡"), |s| TinToken::Fn(s.to_string(), tin_drop)),
        (from_re(r"!"), |s| TinToken::Fn(s.to_string(), tin_dup)),
        (from_re(r"↶"), |s| TinToken::Fn(s.to_string(), tin_swap)),
        (from_re(r"↷"), |s| TinToken::Fn(s.to_string(), tin_copy)),

        (from_re(r"→[a-z_]+"), |s| TinToken::Fn(s[3..].to_string(), tin_define_var)),
        (from_re(r"→\.[a-z_]+"), |s| TinToken::Fn(s[4..].to_string(), tin_redefine_var)),
        (from_re(r"←[a-z_]+"), |s| TinToken::Fn(s[3..].to_string(), tin_delete_var)),
        (from_re(r"\.[a-z_]+"), |s| TinToken::Fn(s[1..].to_string(), tin_get_var)),

        (from_re(r"\|[^|]+\|→\|[^|]+\|"), |s| TinToken::Def(s.to_string())),
        (from_fn(parse_block), |s| TinToken::Fn(s.to_string(), block)),
        (from_fn(parse_rec_block), |s| TinToken::Fn(s.to_string(), rec_block)),

        (from_re(r"\?"), |s| TinToken::Fn(s.to_string(), tin_skip)),
        (from_re(r"◊"), |s| TinToken::Fn(s.to_string(), tin_skip_dup)),
        (from_re(r":"), |s| TinToken::Fn(s.to_string(), tin_skip_inv)),
        (from_re(r"\{"), |s| TinToken::Fn(s.to_string(), tin_foreach_init)),
        (from_re(r"\}"), |s| TinToken::Fn(s.to_string(), tin_foreach_end)),
        (from_re(r"\("), |s| TinToken::Fn(s.to_string(), tin_storer_init)),
        (from_re(r"\)"), |s| TinToken::Fn(s.to_string(), tin_storer_end)),
        (from_re(r"\["), |s| TinToken::Fn(s.to_string(), tin_map_init)),
        (from_re(r"\]"), |s| TinToken::Fn(s.to_string(), tin_map_end)),

        (from_re(r"∇"), |s| TinToken::Fn(s.to_string(), nabla)),

        // Basic arithmetic
        (from_re(r"\+"), |s| TinToken::Fn(s.to_string(), tin_sum)),
        (from_re(r"\-"), |s| TinToken::Fn(s.to_string(), tin_sub)),
        (from_re(r"·"), |s| TinToken::Fn(s.to_string(), tin_mul)),
        (from_re(r"/"), |s| TinToken::Fn(s.to_string(), tin_div)),
        (from_re(r"%"), |s| TinToken::Fn(s.to_string(), tin_mod)),
        (from_re(r"\^"), |s| TinToken::Fn(s.to_string(), tin_pow)),

        (from_re(r"√"), |s| TinToken::Fn(s.to_string(), tin_sqrt)),

        (from_re(r"⊳"), |s| TinToken::Fn(s.to_string(), tin_inc)),
        (from_re(r"⊲"), |s| TinToken::Fn(s.to_string(), tin_dec)),

        (from_re(r"⌉"), |s| TinToken::Fn(s.to_string(), tin_ceil)),
        (from_re(r"⌋"), |s| TinToken::Fn(s.to_string(), tin_floor)),

        // Logic
        (from_re(r"𝔹"), |s| TinToken::Fn(s.to_string(), tin_truthy)),

        (from_re(r"¬"), |s| TinToken::Fn(s.to_string(), tin_neg)),
        (from_re(r"∨"), |s| TinToken::Fn(s.to_string(), tin_or)),
        (from_re(r"∧"), |s| TinToken::Fn(s.to_string(), tin_and)),

        (from_re(r"="), |s| TinToken::Fn(s.to_string(), tin_eq)),
        (from_re(r"<"), |s| TinToken::Fn(s.to_string(), tin_lt)),
        (from_re(r">"), |s| TinToken::Fn(s.to_string(), tin_gt)),

        (from_re(r"∃"), |s| TinToken::Fn(s.to_string(), tin_any)),
        (from_re(r"∄"), |s| TinToken::Fn(s.to_string(), tin_none)),
        (from_re(r"∀"), |s| TinToken::Fn(s.to_string(), tin_all)),

        // Array operations
        (from_re(r"ι"), |s| TinToken::Fn(s.to_string(), iota)),
        (from_re(r"□"), |s| TinToken::Fn(s.to_string(), boxed)),
        (from_re(r"↓"), |s| TinToken::Fn(s.to_string(), get)),
        (from_re(r"\*↓"), |s| TinToken::Fn(s.to_string(), get_nc)),
        (from_re(r"↑"), |s| TinToken::Fn(s.to_string(), set)),

        (from_re(r"∑"), |s| TinToken::Fn(s.to_string(), tin_sum_all)),
        (from_re(r"∏"), |s| TinToken::Fn(s.to_string(), tin_mul_all)),

        (from_re(r"⍴"), |s| TinToken::Fn(s.to_string(), tin_len)),

        (from_re(r"⌈"), |s| TinToken::Fn(s.to_string(), tin_max)),
        (from_re(r"⌊"), |s| TinToken::Fn(s.to_string(), tin_min)),

        (from_re(r"#"), |s| TinToken::Fn(s.to_string(), tin_count)),
        (from_re(r"\*#"), |s| TinToken::Fn(s.to_string(), tin_nc_count)),
        (from_re(r"º"), |s| TinToken::Fn(s.to_string(), tin_index)),
        (from_re(r"\*º"), |s| TinToken::Fn(s.to_string(), tin_nc_index)),
        (from_re(r"@"), |s| TinToken::Fn(s.to_string(), tin_from_index)),
        
        (from_re(r"⇑"), |s| TinToken::Fn(s.to_string(), tin_sort_asc)),
        (from_re(r"\.⇑"), |s| TinToken::Fn(s.to_string(), tin_sort_idx_asc)),
        (from_re(r"⇓"), |s| TinToken::Fn(s.to_string(), tin_sort_desc)),
        (from_re(r"\.⇓"), |s| TinToken::Fn(s.to_string(), tin_sort_idx_desc)),

        (from_re(r"⊃"), |s| TinToken::Fn(s.to_string(), tin_unique)),
        (from_re(r"⊂"), |s| TinToken::Fn(s.to_string(), tin_counts)),

        (from_re(r","), |s| TinToken::Fn(s.to_string(), tin_append)),
        (from_re(r"_"), |s| TinToken::Fn(s.to_string(), tin_merge)),

        // Functional array manipulation
        (from_re(r"`"), |s| TinToken::Fn(s.to_string(), drop_first)),
        (from_re(r"´"), |s| TinToken::Fn(s.to_string(), drop_last)),

        // IO
        (from_re(r"\$"), |s| TinToken::Fn(s.to_string(), tin_print))
    );

    return res;
}