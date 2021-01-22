use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

use crate::wrappers;
use crate::interpreter::{*};
use crate::parallelism;

fn tin_drop(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    stack.pop();
}

fn tin_dup(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    stack.push(stack.last().cloned().unwrap());
}

fn tin_swap(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let last_index = stack.len() - 1;
    stack.swap(last_index, last_index - 1);
}

fn tin_copy(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    if let TinValue::Int(n) = stack.pop().unwrap() {
        let item = stack.get(stack.len() - 1 - n as usize).cloned().unwrap();
        stack.push(item);
    
    } else {
        panic!();
    }
}

fn tin_define_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let ctx = intrp.variables.entry(tok).or_insert(vec!());
    ctx.push(stack.pop().unwrap());
}

fn tin_redefine_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let ctx = intrp.variables.entry(tok).or_insert(vec!());
    
    if ctx.len() > 0 {
        *ctx.last_mut().unwrap() = stack.pop().unwrap();

    } else {
        ctx.push(stack.pop().unwrap());
    }
}

fn tin_delete_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, _stack: &mut Vec<TinValue>){
    if intrp.variables.contains_key(&tok.to_string()) {
        let ctx = intrp.variables.get_mut(&tok).unwrap();
        ctx.pop();

        if ctx.len() == 0 {
            intrp.variables.remove(&tok);
        }
    }
}

fn tin_get_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let ctx = &intrp.variables[&tok];
    stack.push(ctx.last().cloned().unwrap());
}

fn tin_skip(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>){
    let last_t = stack.pop().unwrap().truthy();

    if !last_t{
        *ip += 1;
    }
}

fn tin_skip_dup(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>){
    let last_t = stack.last().as_ref().unwrap().truthy();

    if !last_t{
        *ip += 1;
    }
}

fn tin_skip_inv(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>){
    let last_t = stack.pop().unwrap().truthy();

    if last_t{
        *ip += 1;
    }
}

fn tin_foreach_init(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>){
    if !intrp.loop_stack.is_empty() && intrp.loop_stack.last().unwrap().0 == *ip{
        intrp.loop_stack.last_mut().unwrap().2 += 1;

    } else{
        match stack.pop().unwrap(){
            TinValue::Vector(v) => intrp.loop_stack.push((*ip, v, 0)),

            _ => unreachable!()
        }
    }

    stack.push(intrp.loop_stack.last().unwrap().1[intrp.loop_stack.last().unwrap().2].clone());
}

fn tin_foreach_end(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, _stack: &mut Vec<TinValue>){
    let (pos, arr, idx) = intrp.loop_stack.last().unwrap();

    if *idx < arr.len() - 1 {
        *ip = pos - 1;

    } else{
        intrp.loop_stack.pop();
    }
}

fn tin_storer_init(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    intrp.storer_stack.push(stack.len());
}

fn tin_storer_end(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let idx = intrp.storer_stack.pop().unwrap();
    
    let arr = TinValue::Vector(stack.drain(idx..).collect::<Vec<_>>());
    stack.push(arr);
}

fn tin_map_init(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>){
    if !intrp.map_stack.is_empty() && intrp.map_stack.last().unwrap().0 == *ip{
        intrp.map_stack.last_mut().unwrap().2 += 1;

    } else{
        match stack.pop().unwrap(){
            TinValue::Vector(v) => intrp.map_stack.push((*ip, v, 0, stack.len(), vec!())),

            _ => unreachable!()
        }
    }

    stack.push(intrp.map_stack.last().unwrap().1[intrp.map_stack.last().unwrap().2].clone());
}

fn tin_map_end(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut i64, stack: &mut Vec<TinValue>){
    let (pos, arr, idx, stack_pos, acum) = intrp.map_stack.last_mut().unwrap();

    acum.append(&mut stack.drain(*stack_pos..).collect());

    if *idx < arr.len() - 1 {
        *ip = *pos - 1;

    } else{
        let (_, _, _, _, acum) = intrp.map_stack.pop().unwrap();
        stack.push(TinValue::Vector(acum));
    }
}

fn nabla(_tok: String, intrp: &mut TinInterpreter, prog: &Vec<TinToken>, prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    if prog_parent.is_some(){
        intrp.execute(prog_parent.unwrap(), Option::None, stack);

    } else{
        intrp.execute(prog, prog_parent, stack);
    }
}

fn block(tok: String, intrp: &mut TinInterpreter, prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let new_tok = &tok[3..tok.len() - 3];
    let program = intrp.parse(new_tok);
    intrp.execute(&program, Option::Some(prog), stack);
}

fn tin_eq(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.pop().unwrap();
    let b = stack.last().unwrap();

    *stack.last_mut().unwrap() = TinValue::Int((a == *b) as i64);
}

fn tin_lt(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.pop().unwrap();
    let b = stack.last().unwrap();

    *stack.last_mut().unwrap() = wrappers::lt(&a, &b);
}

fn tin_gt(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.pop().unwrap();
    let b = stack.last().unwrap();

    *stack.last_mut().unwrap() = wrappers::gt(&a, &b);
}

fn tin_sum(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.pop().unwrap();
    let b = stack.last().unwrap();

    *stack.last_mut().unwrap() = wrappers::sum(&a, &b);
}

fn tin_sub(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.pop().unwrap();
    let b = stack.last().unwrap();

    *stack.last_mut().unwrap() = wrappers::sub(&a, &b);
}

fn tin_mul(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.pop().unwrap();
    let b = stack.last().unwrap();

    *stack.last_mut().unwrap() = wrappers::mul(&a, &b);
}

fn tin_div(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.pop().unwrap();
    let b = stack.last().unwrap();

    *stack.last_mut().unwrap() = wrappers::div(&a, &b);
}

fn tin_mod(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.pop().unwrap();
    let b = stack.last().unwrap();

    *stack.last_mut().unwrap() = wrappers::modl(&a, &b);
}

fn tin_pow(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.pop().unwrap();
    let b = stack.last().unwrap();

    *stack.last_mut().unwrap() = wrappers::pow(&a, &b);
}

fn tin_sqrt(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.last().unwrap();

    *stack.last_mut().unwrap() = wrappers::sqrt(&a);
}

fn tin_inc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let one = TinValue::Int(1);

    *stack.last_mut().unwrap() = wrappers::sum(&stack.last().unwrap(), &one);
}

fn tin_dec(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let one = TinValue::Int(1);

    *stack.last_mut().unwrap() = wrappers::sub(&stack.last().unwrap(), &one);
}

fn tin_floor(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    *stack.last_mut().unwrap() = wrappers::floor(&stack.last().unwrap());
}

fn tin_ceil(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    *stack.last_mut().unwrap() = wrappers::ceil(&stack.last().unwrap());

}

fn tin_truthy(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    *stack.last_mut().unwrap() = wrappers::truthy(&stack.last().unwrap());
}

fn tin_neg(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    *stack.last_mut().unwrap() = wrappers::neg(&stack.last().unwrap());
}

fn tin_or(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.pop().unwrap();
    let b = stack.last().unwrap();

    *stack.last_mut().unwrap() = wrappers::or(&a, &b);
}

fn tin_and(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let a = stack.pop().unwrap();
    let b = stack.last().unwrap();

    *stack.last_mut().unwrap() = wrappers::and(&a, &b);
}

fn tin_any(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let res = match stack.pop().unwrap(){
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_any(v)

            } else{
                TinValue::Int(v.iter().any(TinValue::truthy) as i64)
            }
        },

        _ => unreachable!()
    };
    
    stack.push(res);
}

fn tin_none(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let res = match stack.pop().unwrap(){
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_none(v)

            } else{
                TinValue::Int(!v.iter().any(TinValue::truthy) as i64)
            }
        },

        _ => unreachable!()
    };
    
    stack.push(res);
}

fn tin_all(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let res = match stack.pop().unwrap(){
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_all(v)

            } else{
                TinValue::Int(v.iter().all(TinValue::truthy) as i64)
            }
        },
        
        _ => unreachable!()
    };
    
    stack.push(res);
}

fn iota(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let res = match stack.pop().unwrap() {
        TinValue::Int(a) => TinValue::Vector((0..a).map(TinValue::Int).collect::<Vec<_>>()),
        TinValue::Float(a) => TinValue::Vector((0..a as i64).map(TinValue::Int).collect::<Vec<_>>()),

        _ => unreachable!()
    };
    
    stack.push(res);
}

fn boxed(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let res = TinValue::Vector(vec!(stack.pop().unwrap()));
    stack.push(res);
}

fn set(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let idx = stack.pop().unwrap();
    let elem = stack.pop().unwrap();
    let v = stack.last_mut().unwrap();

    match (idx, v) {
        (TinValue::Int(a), TinValue::Vector(v)) => v[a as usize] = elem,

        _ => unreachable!()
    };
}

fn get(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let idx = stack.pop().unwrap();
    let v = stack.pop().unwrap();

    let res = match (idx, v) {
        (TinValue::Int(a), TinValue::Vector(v)) => v[a as usize].clone(),

        _ => unreachable!()
    };
    
    stack.push(res);
}

fn get_nc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let idx = stack.pop().unwrap();
    let v = stack.last().unwrap();

    let res = match (idx, v) {
        (TinValue::Int(a), TinValue::Vector(v)) => v[a as usize].clone(),

        _ => unreachable!()
    };
    
    stack.push(res);
}

fn tin_sum_all(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    match stack.pop().unwrap(){
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

        _ => unreachable!()
    };
}

fn tin_mul_all(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    match stack.pop().unwrap(){
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

        _ => unreachable!()
    };
}

fn tin_len(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let res = match stack.pop().unwrap() {
        TinValue::Vector(v) => TinValue::Int(v.len() as i64),

        _ => unreachable!()
    };
    
    stack.push(res);
}

fn tin_max(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    match stack.pop().unwrap() {
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

        _ => unreachable!()
    };
}

fn tin_min(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    match stack.pop().unwrap() {
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

        _ => unreachable!()
    };
}

fn tin_count(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let elem = stack.pop().unwrap();
    
    match stack.pop().unwrap() {
        TinValue::Vector(v) => {
            let mut res = 0;

            for i in v{
                if i == elem {
                    res += 1;
                }
            }

            stack.push(TinValue::Int(res));
        }

        _ => unreachable!()
    };
}

fn tin_nc_count(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let elem = stack.pop().unwrap();
    
    match stack.last().unwrap() {
        TinValue::Vector(v) => {
            let mut res = 0;

            for i in v{
                if *i == elem {
                    res += 1;
                }
            }

            stack.push(TinValue::Int(res));
        }

        _ => unreachable!()
    };
}

fn tin_index(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let elem = stack.pop().unwrap();
    
    match stack.pop().unwrap() {
        TinValue::Vector(v) => {
            let mut res = vec!();

            for (idx, i) in v.iter().enumerate(){
                if *i == elem {
                    res.push(TinValue::Int(idx as i64));
                }
            }

            stack.push(TinValue::Vector(res));
        }

        _ => unreachable!()
    };
}

fn tin_nc_index(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let elem = stack.pop().unwrap();
    
    match stack.last().unwrap() {
        TinValue::Vector(v) => {
            let mut res = vec!();

            for (idx, i) in v.iter().enumerate(){
                if *i == elem {
                    res.push(TinValue::Int(idx as i64));
                }
            }

            stack.push(TinValue::Vector(res));
        }

        _ => unreachable!()
    };
}

fn tin_from_index(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    let mut idx_vec = stack.pop().unwrap();
    let mut ref_vec = stack.last_mut().unwrap();
    
    match (&mut ref_vec, &mut idx_vec) {
        (TinValue::Vector(ref_v), TinValue::Vector(idx)) => {
            *ref_v = idx.iter().map(|i| match i {
                TinValue::Int(n) => ref_v[*n as usize].clone(),
                _ => unreachable!()
            }).collect();
        }

        _ => unreachable!()
    };
}

fn tin_sort_asc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){    
    match stack.last_mut().unwrap() {
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_sort_asc(v);
                
            } else{
                v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            }
        }

        _ => unreachable!()
    };
}

fn tin_sort_idx_asc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){    
    match stack.last_mut().unwrap() {
        TinValue::Vector(v) => {
            let mut v_cpy = v.iter().enumerate().collect::<Vec<_>>();
            
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_sort_idx_asc(&mut v_cpy);

            } else{
                v_cpy.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap()); 
            }

            *v = v_cpy.iter().map(|t| TinValue::Int(t.0 as i64)).collect();
        }

        _ => unreachable!()
    };
}

fn tin_sort_desc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){    
    match stack.last_mut().unwrap() {
        TinValue::Vector(v) => {
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_sort_desc(v);
                
            } else{
                v.sort_by(|a, b| b.partial_cmp(a).unwrap()); 
            }
        }

        _ => unreachable!()
    };
}

fn tin_sort_idx_desc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){    
    match stack.last_mut().unwrap() {
        TinValue::Vector(v) => {
            let mut v_cpy = v.iter().enumerate().collect::<Vec<_>>();
            
            if parallelism::parallelizable(v.len()){
                parallelism::parallel_sort_idx_desc(&mut v_cpy);

            } else{
                v_cpy.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap()); 
            }

            *v = v_cpy.iter().map(|t| TinValue::Int(t.0 as i64)).collect();
        }

        _ => unreachable!()
    };
}

fn tin_unique(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){    
    match stack.last_mut().unwrap() {
        TinValue::Vector(v) => {
            *v = v.iter().collect::<BTreeSet<_>>().iter().map(|i| (*i).clone()).collect();
        },

        _ => unreachable!()
    };
}

fn tin_counts(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){    
    match stack.last_mut().unwrap() {
        TinValue::Vector(v) => {
            let mut counts = BTreeMap::new();

            for i in v.iter() {
                counts.entry(i).and_modify(|e| *e += 1).or_insert(1);
            }

            *v = v.iter().map(|i| TinValue::Int(*counts.get(i).unwrap() as i64)).collect();
        },

        _ => unreachable!()
    };
}

fn tin_merge(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){    
    let mut v1 = stack.pop().unwrap();
    let mut v2 = stack.last_mut().unwrap();

    match (&mut v1, &mut v2) {
        (TinValue::Vector(a), TinValue::Vector(b)) => b.append(a),

        _ => unreachable!()
    };
}

fn tin_append(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){    
    let elem = stack.pop().unwrap();

    match stack.last_mut().unwrap() {
        TinValue::Vector(v) => v.push(elem),

        _ => unreachable!()
    };
}

fn drop_first(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    match stack.last_mut().unwrap() {
        TinValue::Vector(v) => {
            if v.len() > 0 {
                v.remove(0);
            }
        },

        _ => unreachable!()
    };
}

fn drop_last(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    match stack.last_mut().unwrap() {
        TinValue::Vector(v) => {
            if v.len() > 0 {
                v.pop();
            }
        },

        _ => unreachable!()
    };
}

fn tin_print(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut i64, stack: &mut Vec<TinValue>){
    println!("{}", stack.pop().unwrap().to_string());
}

pub fn std_tin_functions() -> Vec<(Regex, fn(&str) -> TinToken)>{
    let res_str: Vec<(&str, fn(&str) -> TinToken)> = vec!(

        // Literals
        (r"\d+", |s| TinToken::Int(s.parse::<i64>().unwrap())),
        (r"\d*\.\d+", |s| TinToken::Float(s.parse::<f64>().unwrap())),

        // Meta
        (r"¡", |s| TinToken::Fn(s.to_string(), tin_drop)),
        (r"!", |s| TinToken::Fn(s.to_string(), tin_dup)),
        (r"↶", |s| TinToken::Fn(s.to_string(), tin_swap)),
        (r"↷", |s| TinToken::Fn(s.to_string(), tin_copy)),

        (r"→[a-z_]+", |s| TinToken::Fn(s[3..].to_string(), tin_define_var)),
        (r"→\.[a-z_]+", |s| TinToken::Fn(s[4..].to_string(), tin_redefine_var)),
        (r"←[a-z_]+", |s| TinToken::Fn(s[3..].to_string(), tin_delete_var)),
        (r"\.[a-z_]+", |s| TinToken::Fn(s[1..].to_string(), tin_get_var)),

        (r"\|[^|]+\|→\|[^|]+\|", |s| TinToken::Def(s.to_string())),
        (r"⟨[^⟨⟩]+⟩", |s| TinToken::Fn(s.to_string(), block)),

        (r"\?", |s| TinToken::Fn(s.to_string(), tin_skip)),
        (r"◊", |s| TinToken::Fn(s.to_string(), tin_skip_dup)),
        (r":", |s| TinToken::Fn(s.to_string(), tin_skip_inv)),
        (r"\{", |s| TinToken::Fn(s.to_string(), tin_foreach_init)),
        (r"\}", |s| TinToken::Fn(s.to_string(), tin_foreach_end)),
        (r"\(", |s| TinToken::Fn(s.to_string(), tin_storer_init)),
        (r"\)", |s| TinToken::Fn(s.to_string(), tin_storer_end)),
        (r"\[", |s| TinToken::Fn(s.to_string(), tin_map_init)),
        (r"\]", |s| TinToken::Fn(s.to_string(), tin_map_end)),

        (r"∇", |s| TinToken::Fn(s.to_string(), nabla)),

        // Basic arithmetic
        (r"\+", |s| TinToken::Fn(s.to_string(), tin_sum)),
        (r"\-", |s| TinToken::Fn(s.to_string(), tin_sub)),
        (r"·", |s| TinToken::Fn(s.to_string(), tin_mul)),
        (r"/", |s| TinToken::Fn(s.to_string(), tin_div)),
        (r"%", |s| TinToken::Fn(s.to_string(), tin_mod)),
        (r"\^", |s| TinToken::Fn(s.to_string(), tin_pow)),

        (r"√", |s| TinToken::Fn(s.to_string(), tin_sqrt)),

        (r"⊳", |s| TinToken::Fn(s.to_string(), tin_inc)),
        (r"⊲", |s| TinToken::Fn(s.to_string(), tin_dec)),

        (r"⌉", |s| TinToken::Fn(s.to_string(), tin_ceil)),
        (r"⌋", |s| TinToken::Fn(s.to_string(), tin_floor)),

        // Logic
        (r"𝔹", |s| TinToken::Fn(s.to_string(), tin_truthy)),

        (r"¬", |s| TinToken::Fn(s.to_string(), tin_neg)),
        (r"∨", |s| TinToken::Fn(s.to_string(), tin_or)),
        (r"∧", |s| TinToken::Fn(s.to_string(), tin_and)),

        (r"=", |s| TinToken::Fn(s.to_string(), tin_eq)),
        (r"<", |s| TinToken::Fn(s.to_string(), tin_lt)),
        (r">", |s| TinToken::Fn(s.to_string(), tin_gt)),

        (r"∃", |s| TinToken::Fn(s.to_string(), tin_any)),
        (r"∄", |s| TinToken::Fn(s.to_string(), tin_none)),
        (r"∀", |s| TinToken::Fn(s.to_string(), tin_all)),

        // Array operations
        (r"ι", |s| TinToken::Fn(s.to_string(), iota)),
        (r"□", |s| TinToken::Fn(s.to_string(), boxed)),
        (r"↓", |s| TinToken::Fn(s.to_string(), get)),
        (r"\*↓", |s| TinToken::Fn(s.to_string(), get_nc)),
        (r"↑", |s| TinToken::Fn(s.to_string(), set)),

        (r"∑", |s| TinToken::Fn(s.to_string(), tin_sum_all)),
        (r"∏", |s| TinToken::Fn(s.to_string(), tin_mul_all)),

        (r"⍴", |s| TinToken::Fn(s.to_string(), tin_len)),

        (r"⌈", |s| TinToken::Fn(s.to_string(), tin_max)),
        (r"⌊", |s| TinToken::Fn(s.to_string(), tin_min)),

        (r"#", |s| TinToken::Fn(s.to_string(), tin_count)),
        (r"\*#", |s| TinToken::Fn(s.to_string(), tin_nc_count)),
        (r"º", |s| TinToken::Fn(s.to_string(), tin_index)),
        (r"\*º", |s| TinToken::Fn(s.to_string(), tin_nc_index)),
        (r"@", |s| TinToken::Fn(s.to_string(), tin_from_index)),
        
        (r"⇑", |s| TinToken::Fn(s.to_string(), tin_sort_asc)),
        (r"\.⇑", |s| TinToken::Fn(s.to_string(), tin_sort_idx_asc)),
        (r"⇓", |s| TinToken::Fn(s.to_string(), tin_sort_desc)),
        (r"\.⇓", |s| TinToken::Fn(s.to_string(), tin_sort_idx_desc)),

        (r"⊃", |s| TinToken::Fn(s.to_string(), tin_unique)),
        (r"⊂", |s| TinToken::Fn(s.to_string(), tin_counts)),

        (r",", |s| TinToken::Fn(s.to_string(), tin_append)),
        (r"_", |s| TinToken::Fn(s.to_string(), tin_merge)),

        // Functional array manipulation
        (r"`", |s| TinToken::Fn(s.to_string(), drop_first)),
        (r"´", |s| TinToken::Fn(s.to_string(), drop_last)),

        // IO
        (r"\$", |s| TinToken::Fn(s.to_string(), tin_print))
    );

    return res_str.iter().map(|t| (Regex::new(t.0).unwrap(), t.1)).collect::<Vec<_>>();
}