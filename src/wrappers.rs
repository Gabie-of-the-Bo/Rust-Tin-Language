use rayon::prelude::*;

use crate::interpreter::{*};
use crate::parallelism::parallelizable;

pub fn floor(a: &TinValue) -> TinValue{
    return match a{
        TinValue::Int(n) => TinValue::Int(*n),
        TinValue::Float(n) => TinValue::Int(n.floor() as i64),
        TinValue::Vector(v) => if parallelizable(v.len()) {TinValue::Vector(v.par_iter().map(floor).collect())}
                                                     else {TinValue::Vector(v.iter().map(floor).collect())},
    }
}

pub fn ceil(a: &TinValue) -> TinValue{
    return match a{
        TinValue::Int(n) => TinValue::Int(*n),
        TinValue::Float(n) => TinValue::Int(n.ceil() as i64),
        TinValue::Vector(v) => if parallelizable(v.len()) {TinValue::Vector(v.par_iter().map(ceil).collect())}
                                                     else {TinValue::Vector(v.iter().map(ceil).collect())},
    }
}

pub fn truthy(a: &TinValue) -> TinValue{
    return match a{
        TinValue::Vector(v) => if parallelizable(v.len()) {TinValue::Vector(v.par_iter().map(truthy).collect())}
                                                     else {TinValue::Vector(v.iter().map(truthy).collect())},

        _ => TinValue::Int(a.truthy() as i64)
    }
}

pub fn neg(a: &TinValue) -> TinValue{
    return match a{
        TinValue::Vector(v) => if parallelizable(v.len()) {TinValue::Vector(v.par_iter().map(neg).collect())}
                                                     else {TinValue::Vector(v.iter().map(neg).collect())},

        _ => TinValue::Int((!a.truthy()) as i64)
    }
}

pub fn or(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::Vector(a), TinValue::Vector(b)) => if parallelizable(a.len()) {TinValue::Vector(a.par_iter().zip(b).map(|t| or(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::Vector(a.iter().zip(b).map(|t| or(t.0, t.1)).collect::<Vec<_>>())},

        _ => TinValue::Int((aa.truthy() || bb.truthy()) as i64)
    };
}

pub fn and(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::Vector(a), TinValue::Vector(b)) => if parallelizable(a.len()) {TinValue::Vector(a.par_iter().zip(b).map(|t| and(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::Vector(a.iter().zip(b).map(|t| and(t.0, t.1)).collect::<Vec<_>>())},

        _ => TinValue::Int((aa.truthy() && bb.truthy()) as i64)
    };
}

pub fn lt(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::Int(a), TinValue::Int(b)) => TinValue::Int((a < b) as i64),
        (TinValue::Int(a), TinValue::Float(b)) => TinValue::Int((*b > *a as f64) as i64),
        (TinValue::Float(a), TinValue::Int(b)) => TinValue::Int((*a < *b as f64) as i64),
        (TinValue::Float(a), TinValue::Float(b)) => TinValue::Int((a < b) as i64),

        (TinValue::Int(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| lt(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| lt(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Int(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| lt(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| lt(v, bb)).collect::<Vec<_>>())},

        (TinValue::Float(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| lt(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| lt(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Float(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| lt(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| lt(v, bb)).collect::<Vec<_>>())},

        (TinValue::Vector(a), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|t| lt(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::Vector(a.iter().zip(b).map(|t| lt(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn gt(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::Int(a), TinValue::Int(b)) => TinValue::Int((a > b) as i64),
        (TinValue::Int(a), TinValue::Float(b)) => TinValue::Int((*a as f64 > *b) as i64),
        (TinValue::Float(a), TinValue::Int(b)) => TinValue::Int((*a > *b as f64) as i64),
        (TinValue::Float(a), TinValue::Float(b)) => TinValue::Int((a > b) as i64),

        (TinValue::Int(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| gt(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| gt(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Int(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| gt(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| gt(v, bb)).collect::<Vec<_>>())},

        (TinValue::Float(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| gt(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| gt(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Float(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| gt(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| gt(v, bb)).collect::<Vec<_>>())},

        (TinValue::Vector(a), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|t| gt(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::Vector(a.iter().zip(b).map(|t| gt(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn sum(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::Int(a), TinValue::Int(b)) => TinValue::Int(a + b),
        (TinValue::Int(a), TinValue::Float(b)) => TinValue::Float(*a as f64 + b),
        (TinValue::Float(a), TinValue::Int(b)) => TinValue::Float(a + *b as f64),
        (TinValue::Float(a), TinValue::Float(b)) => TinValue::Float(a + b),

        (TinValue::Int(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| sum(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| sum(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Int(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| sum(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| sum(v, bb)).collect::<Vec<_>>())},

        (TinValue::Float(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| sum(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| sum(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Float(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| sum(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| sum(v, bb)).collect::<Vec<_>>())},

        (TinValue::Vector(a), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|t| sum(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::Vector(a.iter().zip(b).map(|t| sum(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn sub(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::Int(a), TinValue::Int(b)) => TinValue::Int(a - b),
        (TinValue::Int(a), TinValue::Float(b)) => TinValue::Float(*a as f64 - b),
        (TinValue::Float(a), TinValue::Int(b)) => TinValue::Float(a - *b as f64),
        (TinValue::Float(a), TinValue::Float(b)) => TinValue::Float(a - b),

        (TinValue::Int(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| sub(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| sub(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Int(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| sub(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| sub(v, bb)).collect::<Vec<_>>())},

        (TinValue::Float(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| sub(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| sub(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Float(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| sub(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| sub(v, bb)).collect::<Vec<_>>())},

        (TinValue::Vector(a), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|t| sub(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::Vector(a.iter().zip(b).map(|t| sub(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn mul(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::Int(a), TinValue::Int(b)) => TinValue::Int(a * b),
        (TinValue::Int(a), TinValue::Float(b)) => TinValue::Float(*a as f64 * b),
        (TinValue::Float(a), TinValue::Int(b)) => TinValue::Float(a * *b as f64),
        (TinValue::Float(a), TinValue::Float(b)) => TinValue::Float(a * b),

        (TinValue::Int(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| mul(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| mul(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Int(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| mul(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| mul(v, bb)).collect::<Vec<_>>())},

        (TinValue::Float(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| mul(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| mul(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Float(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| mul(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| mul(v, bb)).collect::<Vec<_>>())},

        (TinValue::Vector(a), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|t| mul(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::Vector(a.iter().zip(b).map(|t| mul(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn div(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::Int(a), TinValue::Int(b)) => TinValue::Int(a / b),
        (TinValue::Int(a), TinValue::Float(b)) => TinValue::Float(*a as f64 / b),
        (TinValue::Float(a), TinValue::Int(b)) => TinValue::Float(a / *b as f64),
        (TinValue::Float(a), TinValue::Float(b)) => TinValue::Float(a / b),

        (TinValue::Int(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| div(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| div(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Int(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| div(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| div(v, bb)).collect::<Vec<_>>())},

        (TinValue::Float(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| div(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| div(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Float(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| div(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| div(v, bb)).collect::<Vec<_>>())},

        (TinValue::Vector(a), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|t| div(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::Vector(a.iter().zip(b).map(|t| div(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn modl(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::Int(a), TinValue::Int(b)) => TinValue::Int(a % b),
        (TinValue::Int(a), TinValue::Float(b)) => TinValue::Float(*a as f64 % b),
        (TinValue::Float(a), TinValue::Int(b)) => TinValue::Float(a % *b as f64),
        (TinValue::Float(a), TinValue::Float(b)) => TinValue::Float(a % b),

        (TinValue::Int(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| modl(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| modl(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Int(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| modl(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| modl(v, bb)).collect::<Vec<_>>())},

        (TinValue::Float(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| modl(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| modl(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Float(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| modl(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| modl(v, bb)).collect::<Vec<_>>())},

        (TinValue::Vector(a), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|t| modl(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::Vector(a.iter().zip(b).map(|t| modl(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn pow(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::Int(a), TinValue::Int(b)) => TinValue::Int(a.pow(*b as u32)),
        (TinValue::Int(a), TinValue::Float(b)) => TinValue::Float((*a as f64).powf(*b)),
        (TinValue::Float(a), TinValue::Int(b)) => TinValue::Float(a.powf(*b as f64)),
        (TinValue::Float(a), TinValue::Float(b)) => TinValue::Float(a.powf(*b)),

        (TinValue::Int(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| pow(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| pow(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Int(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| pow(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::Vector(b.iter().map(|v| pow(v, bb)).collect::<Vec<_>>())},

        (TinValue::Float(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| pow(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| pow(aa, v)).collect::<Vec<_>>())},

        (TinValue::Vector(b), TinValue::Float(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| pow(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::Vector(b.iter().map(|v| pow(v, bb)).collect::<Vec<_>>())},

        (TinValue::Vector(a), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|t| pow(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::Vector(a.iter().zip(b).map(|t| pow(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn sqrt(a: &TinValue) -> TinValue{
    return match a{
        TinValue::Int(n) => TinValue::Float((*n as f64).sqrt()),
        TinValue::Float(n) => TinValue::Float(n.sqrt()),
        TinValue::Vector(v) => if parallelizable(v.len()) {TinValue::Vector(v.par_iter().map(sqrt).collect())}
                                                     else {TinValue::Vector(v.iter().map(sqrt).collect())},
    }
}