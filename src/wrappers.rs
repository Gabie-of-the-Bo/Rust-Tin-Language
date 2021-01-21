use rayon::prelude::*;

use crate::interpreter::{*};
use crate::parallelism::parallelizable;

pub fn floor(a: &TinValue) -> TinValue{
    return match a{
        TinValue::INT(n) => TinValue::INT(*n),
        TinValue::FLOAT(n) => TinValue::INT(n.floor() as i64),
        TinValue::VECTOR(v) => if parallelizable(v.len()) {TinValue::VECTOR(v.par_iter().map(floor).collect())}
                                                     else {TinValue::VECTOR(v.iter().map(floor).collect())},
    }
}

pub fn ceil(a: &TinValue) -> TinValue{
    return match a{
        TinValue::INT(n) => TinValue::INT(*n),
        TinValue::FLOAT(n) => TinValue::INT(n.ceil() as i64),
        TinValue::VECTOR(v) => if parallelizable(v.len()) {TinValue::VECTOR(v.par_iter().map(ceil).collect())}
                                                     else {TinValue::VECTOR(v.iter().map(ceil).collect())},
    }
}

pub fn truthy(a: &TinValue) -> TinValue{
    return match a{
        TinValue::VECTOR(v) => if parallelizable(v.len()) {TinValue::VECTOR(v.par_iter().map(truthy).collect())}
                                                     else {TinValue::VECTOR(v.iter().map(truthy).collect())},

        _ => TinValue::INT(a.truthy() as i64)
    }
}

pub fn neg(a: &TinValue) -> TinValue{
    return match a{
        TinValue::VECTOR(v) => if parallelizable(v.len()) {TinValue::VECTOR(v.par_iter().map(neg).collect())}
                                                     else {TinValue::VECTOR(v.iter().map(neg).collect())},

        _ => TinValue::INT((!a.truthy()) as i64)
    }
}

pub fn or(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => if parallelizable(a.len()) {TinValue::VECTOR(a.par_iter().zip(b).map(|t| or(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::VECTOR(a.iter().zip(b).map(|t| or(t.0, t.1)).collect::<Vec<_>>())},

        _ => TinValue::INT((aa.truthy() || bb.truthy()) as i64)
    };
}

pub fn and(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => if parallelizable(a.len()) {TinValue::VECTOR(a.par_iter().zip(b).map(|t| and(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::VECTOR(a.iter().zip(b).map(|t| and(t.0, t.1)).collect::<Vec<_>>())},

        _ => TinValue::INT((aa.truthy() && bb.truthy()) as i64)
    };
}

pub fn lt(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT((a < b) as i64),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::INT((*b > *a as f64) as i64),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::INT((*a < *b as f64) as i64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::INT((a < b) as i64),

        (TinValue::INT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| lt(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| lt(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::INT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| lt(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| lt(v, bb)).collect::<Vec<_>>())},

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| lt(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| lt(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| lt(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| lt(v, bb)).collect::<Vec<_>>())},

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(a.par_iter().zip(b).map(|t| lt(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::VECTOR(a.iter().zip(b).map(|t| lt(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn gt(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT((a > b) as i64),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::INT((*a as f64 > *b) as i64),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::INT((*a > *b as f64) as i64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::INT((a > b) as i64),

        (TinValue::INT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| gt(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| gt(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::INT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| gt(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| gt(v, bb)).collect::<Vec<_>>())},

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| gt(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| gt(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| gt(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| gt(v, bb)).collect::<Vec<_>>())},

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(a.par_iter().zip(b).map(|t| gt(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::VECTOR(a.iter().zip(b).map(|t| gt(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn sum(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT(a + b),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(*a as f64 + b),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::FLOAT(a + *b as f64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(a + b),

        (TinValue::INT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| sum(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| sum(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::INT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| sum(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| sum(v, bb)).collect::<Vec<_>>())},

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| sum(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| sum(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| sum(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| sum(v, bb)).collect::<Vec<_>>())},

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(a.par_iter().zip(b).map(|t| sum(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::VECTOR(a.iter().zip(b).map(|t| sum(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn sub(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT(a - b),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(*a as f64 - b),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::FLOAT(a - *b as f64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(a - b),

        (TinValue::INT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| sub(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| sub(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::INT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| sub(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| sub(v, bb)).collect::<Vec<_>>())},

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| sub(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| sub(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| sub(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| sub(v, bb)).collect::<Vec<_>>())},

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(a.par_iter().zip(b).map(|t| sub(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::VECTOR(a.iter().zip(b).map(|t| sub(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn mul(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT(a * b),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(*a as f64 * b),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::FLOAT(a * *b as f64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(a * b),

        (TinValue::INT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| mul(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| mul(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::INT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| mul(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| mul(v, bb)).collect::<Vec<_>>())},

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| mul(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| mul(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| mul(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| mul(v, bb)).collect::<Vec<_>>())},

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(a.par_iter().zip(b).map(|t| mul(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::VECTOR(a.iter().zip(b).map(|t| mul(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn div(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT(a / b),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(*a as f64 / b),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::FLOAT(a / *b as f64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(a / b),

        (TinValue::INT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| div(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| div(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::INT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| div(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| div(v, bb)).collect::<Vec<_>>())},

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| div(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| div(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| div(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| div(v, bb)).collect::<Vec<_>>())},

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(a.par_iter().zip(b).map(|t| div(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::VECTOR(a.iter().zip(b).map(|t| div(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn modl(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT(a % b),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(*a as f64 % b),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::FLOAT(a % *b as f64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(a % b),

        (TinValue::INT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| modl(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| modl(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::INT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| modl(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| modl(v, bb)).collect::<Vec<_>>())},

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| modl(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| modl(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| modl(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| modl(v, bb)).collect::<Vec<_>>())},

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(a.par_iter().zip(b).map(|t| modl(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::VECTOR(a.iter().zip(b).map(|t| modl(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn pow(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT(a.pow(*b as u32)),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::FLOAT((*a as f64).powf(*b)),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::FLOAT(a.powf(*b as f64)),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(a.powf(*b)),

        (TinValue::INT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| pow(aa, v)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| pow(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::INT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| pow(v, bb)).collect::<Vec<_>>())}
                                                                         else {TinValue::VECTOR(b.iter().map(|v| pow(v, bb)).collect::<Vec<_>>())},

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| pow(aa, v)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| pow(aa, v)).collect::<Vec<_>>())},

        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => if parallelizable(b.len()) {TinValue::VECTOR(b.par_iter().map(|v| pow(v, bb)).collect::<Vec<_>>())}
                                                                           else {TinValue::VECTOR(b.iter().map(|v| pow(v, bb)).collect::<Vec<_>>())},

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => if parallelizable(b.len()) {TinValue::VECTOR(a.par_iter().zip(b).map(|t| pow(t.0, t.1)).collect::<Vec<_>>())}
                                                                            else {TinValue::VECTOR(a.iter().zip(b).map(|t| pow(t.0, t.1)).collect::<Vec<_>>())},
    };
}

pub fn sqrt(a: &TinValue) -> TinValue{
    return match a{
        TinValue::INT(n) => TinValue::FLOAT((*n as f64).sqrt()),
        TinValue::FLOAT(n) => TinValue::FLOAT(n.sqrt()),
        TinValue::VECTOR(v) => if parallelizable(v.len()) {TinValue::VECTOR(v.par_iter().map(sqrt).collect())}
                                                     else {TinValue::VECTOR(v.iter().map(sqrt).collect())},
    }
}