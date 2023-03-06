use rayon::prelude::*;

use crate::interpreter::{*};
use crate::parallelism::parallelizable;

macro_rules! unary_vectorized_op {
    ($name: ident, $n: ident, $variant_int: ident, $op_int: expr, $variant_float: ident, $op_float: expr, $n_v: ident, $variant_int_v: ident, $variant_float_v: ident) => {
        pub fn $name(a: &TinValue) -> TinValue{
            return match a{
                TinValue::Int($n) => TinValue::$variant_int($op_int),
                TinValue::Float($n) => TinValue::$variant_float($op_float),
                TinValue::Vector(v) => if parallelizable(v.len()) {TinValue::Vector(v.par_iter().map($name).collect())}
                                                             else {TinValue::Vector(v.iter().map($name).collect())},
                
                TinValue::IntVector(v) => if parallelizable(v.len()) {TinValue::$variant_int_v(v.par_iter().map(|$n_v| $op_int).collect())}
                                                            else {TinValue::$variant_int_v(v.iter().map(|$n_v| $op_int).collect())},
                
                TinValue::FloatVector(v) => if parallelizable(v.len()) {TinValue::$variant_float_v(v.par_iter().map(|$n_v| $op_float).collect())}
                                                            else {TinValue::$variant_float_v(v.iter().map(|$n_v| $op_float).collect())},
            }
        }
    };
}

macro_rules! binary_vectorized_op {
    ($name: ident, $a: ident, $b: ident, $variant_int: ident, $op_int: expr, $variant_mix_1: ident, $op_mix_1: expr, $variant_mix_2: ident, $op_mix_2: expr, $variant_float: ident, $op_float: expr, $variant_int_v: ident, $variant_float_v: ident, $variant_mix_v: ident) => {
        pub fn $name(aa: &TinValue, bb: &TinValue) -> TinValue{
            return match (aa, bb) {
                (TinValue::Int($a), TinValue::Int($b)) => TinValue::$variant_int($op_int),
                (TinValue::Int($a), TinValue::Float($b)) => TinValue::$variant_mix_1($op_mix_1),
                (TinValue::Float($a), TinValue::Int($b)) => TinValue::$variant_mix_2($op_mix_2),
                (TinValue::Float($a), TinValue::Float($b)) => TinValue::$variant_float($op_float),
        
                // Vector
                (TinValue::Int(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| $name(aa, v)).collect::<Vec<_>>())}
                                                                                 else {TinValue::Vector(b.iter().map(|v| $name(aa, v)).collect::<Vec<_>>())},
        
                (TinValue::Vector(b), TinValue::Int(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| $name(v, bb)).collect::<Vec<_>>())}
                                                                                 else {TinValue::Vector(b.iter().map(|v| $name(v, bb)).collect::<Vec<_>>())},
        
                (TinValue::Float(_), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| $name(aa, v)).collect::<Vec<_>>())}
                                                                                   else {TinValue::Vector(b.iter().map(|v| $name(aa, v)).collect::<Vec<_>>())},
        
                (TinValue::Vector(b), TinValue::Float(_)) => if parallelizable(b.len()) {TinValue::Vector(b.par_iter().map(|v| $name(v, bb)).collect::<Vec<_>>())}
                                                                                   else {TinValue::Vector(b.iter().map(|v| $name(v, bb)).collect::<Vec<_>>())},
        
                (TinValue::Vector(a), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|t| $name(t.0, t.1)).collect::<Vec<_>>())}
                                                                                    else {TinValue::Vector(a.iter().zip(b).map(|t| $name(t.0, t.1)).collect::<Vec<_>>())},

                // Int vector
                (TinValue::Int($a), TinValue::IntVector(v)) => if parallelizable(v.len()) {TinValue::$variant_int_v(v.par_iter().map(|$b| $op_int).collect::<Vec<_>>())}
                                                                                     else {TinValue::$variant_int_v(v.iter().map(|$b| $op_int).collect::<Vec<_>>())},

                (TinValue::IntVector(v), TinValue::Int($b)) => if parallelizable(v.len()) {TinValue::$variant_int_v(v.par_iter().map(|$a| $op_int).collect::<Vec<_>>())}
                                                                                else {TinValue::$variant_int_v(v.iter().map(|$a| $op_int).collect::<Vec<_>>())},

                (TinValue::Float($a), TinValue::IntVector(v)) => if parallelizable(v.len()) {TinValue::$variant_mix_v(v.par_iter().map(|$b| $op_mix_2).collect::<Vec<_>>())}
                                                                                    else {TinValue::$variant_mix_v(v.iter().map(|$b| $op_mix_2).collect::<Vec<_>>())},

                (TinValue::IntVector(v), TinValue::Float($b)) => if parallelizable(v.len()) {TinValue::$variant_mix_v(v.par_iter().map(|$a| $op_mix_1).collect::<Vec<_>>())}
                                                                                    else {TinValue::$variant_mix_v(v.iter().map(|$a| $op_mix_1).collect::<Vec<_>>())},

                (TinValue::IntVector(a), TinValue::IntVector(b)) => if parallelizable(b.len()) {TinValue::$variant_int_v(a.par_iter().zip(b).map(|($a, $b)| $op_int).collect::<Vec<_>>())}
                                                                                    else {TinValue::$variant_int_v(a.iter().zip(b).map(|($a, $b)| $op_int).collect::<Vec<_>>())},

                // Float vector
                (TinValue::Int($a), TinValue::FloatVector(v)) => if parallelizable(v.len()) {TinValue::$variant_float_v(v.par_iter().map(|$b| $op_mix_1).collect::<Vec<_>>())}
                                                                                        else {TinValue::$variant_float_v(v.iter().map(|$b| $op_mix_1).collect::<Vec<_>>())},

                (TinValue::FloatVector(v), TinValue::Int($b)) => if parallelizable(v.len()) {TinValue::$variant_float_v(v.par_iter().map(|$a| $op_mix_2).collect::<Vec<_>>())}
                                                                                else {TinValue::$variant_float_v(v.iter().map(|$a| $op_mix_2).collect::<Vec<_>>())},

                (TinValue::Float($a), TinValue::FloatVector(v)) => if parallelizable(v.len()) {TinValue::$variant_mix_v(v.par_iter().map(|$b| $op_float).collect::<Vec<_>>())}
                                                                                    else {TinValue::$variant_mix_v(v.iter().map(|$b| $op_float).collect::<Vec<_>>())},

                (TinValue::FloatVector(v), TinValue::Float($b)) => if parallelizable(v.len()) {TinValue::$variant_mix_v(v.par_iter().map(|$a| $op_float).collect::<Vec<_>>())}
                                                                                    else {TinValue::$variant_mix_v(v.iter().map(|$a| $op_float).collect::<Vec<_>>())},

                (TinValue::FloatVector(a), TinValue::FloatVector(b)) => if parallelizable(b.len()) {TinValue::$variant_float_v(a.par_iter().zip(b).map(|($a, $b)| $op_float).collect::<Vec<_>>())}
                                                                                    else {TinValue::$variant_float_v(a.iter().zip(b).map(|($a, $b)| $op_float).collect::<Vec<_>>())},

                // Vectors
                (TinValue::FloatVector(a), TinValue::IntVector(b)) => if parallelizable(b.len()) {TinValue::$variant_mix_v(a.par_iter().zip(b).map(|($a, $b)| $op_mix_2).collect::<Vec<_>>())}
                                                                                    else {TinValue::$variant_mix_v(a.iter().zip(b).map(|($a, $b)| $op_mix_2).collect::<Vec<_>>())},

                (TinValue::IntVector(a), TinValue::FloatVector(b)) => if parallelizable(b.len()) {TinValue::$variant_mix_v(a.par_iter().zip(b).map(|($a, $b)| $op_mix_1).collect::<Vec<_>>())}
                                                                                    else {TinValue::$variant_mix_v(a.iter().zip(b).map(|($a, $b)| $op_mix_1).collect::<Vec<_>>())},

                (TinValue::Vector(a), TinValue::FloatVector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|(a, b)| $name(a, &TinValue::Float(*b))).collect::<Vec<_>>())}
                                                                                    else {TinValue::Vector(a.iter().zip(b).map(|(a, b)| $name(a, &TinValue::Float(*b))).collect::<Vec<_>>())},

                (TinValue::FloatVector(a), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|(a, b)| $name(&TinValue::Float(*a), b)).collect::<Vec<_>>())}
                                                                                    else {TinValue::Vector(a.iter().zip(b).map(|(a, b)| $name(&TinValue::Float(*a), b)).collect::<Vec<_>>())},

                (TinValue::Vector(a), TinValue::IntVector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|(a, b)| $name(a, &TinValue::Int(*b))).collect::<Vec<_>>())}
                                                                                    else {TinValue::Vector(a.iter().zip(b).map(|(a, b)| $name(a, &TinValue::Int(*b))).collect::<Vec<_>>())},

                (TinValue::IntVector(a), TinValue::Vector(b)) => if parallelizable(b.len()) {TinValue::Vector(a.par_iter().zip(b).map(|(a, b)| $name(&TinValue::Int(*a), b)).collect::<Vec<_>>())}
                                                                                    else {TinValue::Vector(a.iter().zip(b).map(|(a, b)| $name(&TinValue::Int(*a), b)).collect::<Vec<_>>())},
            };
        }
        
    };
}

// Unary ops
unary_vectorized_op!(floor, n, Int, *n, Int, n.floor() as i64, n, IntVector, IntVector);
unary_vectorized_op!(ceil, n, Int, *n, Int, n.ceil() as i64, n, IntVector, IntVector);
unary_vectorized_op!(truthy, n, Int, (*n != 0) as i64, Int, (*n != 0.0) as i64, n, IntVector, IntVector);
unary_vectorized_op!(neg, n, Int, (*n == 0) as i64, Int, (*n == 0.0) as i64, n, IntVector, IntVector);
unary_vectorized_op!(float, n, Float, *n as f64, Float, *n, n, FloatVector, FloatVector);
unary_vectorized_op!(sqrt, n, Float, (*n as f64).sqrt(), Float, n.sqrt(), n, FloatVector, FloatVector);

// Binary ops
binary_vectorized_op!(or, a, b, Int, (*a != 0 || *b != 0) as i64, Int, (*a != 0 || *b != 0.0) as i64, Int, (*a != 0.0 || *b != 0) as i64, Int, (*a != 0.0 || *b != 0.0) as i64, IntVector, IntVector, IntVector);
binary_vectorized_op!(and, a, b, Int, (*a != 0 && *b != 0) as i64, Int, (*a != 0 && *b != 0.0) as i64, Int, (*a != 0.0 && *b != 0) as i64, Int, (*a != 0.0 && *b != 0.0) as i64, IntVector, IntVector, IntVector);

binary_vectorized_op!(lt, a, b, Int, (*a < *b) as i64, Int, ((*a as f64) < *b) as i64, Int, (*a < *b as f64) as i64, Int, (*a < *b) as i64, IntVector, IntVector, IntVector);
binary_vectorized_op!(leq, a, b, Int, (*a <= *b) as i64, Int, ((*a as f64) <= *b) as i64, Int, (*a <= *b as f64) as i64, Int, (*a <= *b) as i64, IntVector, IntVector, IntVector);
binary_vectorized_op!(gt, a, b, Int, (*a > *b) as i64, Int, ((*a as f64) > *b) as i64, Int, (*a > *b as f64) as i64, Int, (*a > *b) as i64, IntVector, IntVector, IntVector);
binary_vectorized_op!(geq, a, b, Int, (*a >= *b) as i64, Int, ((*a as f64) >= *b) as i64, Int, (*a >= *b as f64) as i64, Int, (*a >= *b) as i64, IntVector, IntVector, IntVector);

binary_vectorized_op!(sum, a, b, Int, *a + *b, Float, (*a as f64) + *b, Float, *a + *b as f64, Float, *a + *b, IntVector, FloatVector, FloatVector);
binary_vectorized_op!(mul, a, b, Int, *a * *b, Float, (*a as f64) * *b, Float, *a * *b as f64, Float, *a * *b, IntVector, FloatVector, FloatVector);

binary_vectorized_op!(sub, a, b, Int, *a - *b, Float, (*a as f64) - *b, Float, *a - *b as f64, Float, *a - *b, IntVector, FloatVector, FloatVector);
binary_vectorized_op!(div, a, b, Int, *a / *b, Float, (*a as f64) / *b, Float, *a / *b as f64, Float, *a / *b, IntVector, FloatVector, FloatVector);
binary_vectorized_op!(modl, a, b, Int, *a % *b, Float, (*a as f64) % *b, Float, *a % *b as f64, Float, *a % *b, IntVector, FloatVector, FloatVector);
binary_vectorized_op!(pow, a, b, Int, a.pow(*b as u32), Float, (*a as f64).powf(*b), Float, a.powf(*b as f64), Float, a.powf(*b), IntVector, FloatVector, FloatVector);