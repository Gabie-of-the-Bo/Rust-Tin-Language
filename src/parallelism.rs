use rayon::prelude::*;

use crate::interpreter::*;
use crate::wrappers;

pub fn parallel_sum_all(vector: Vec<TinValue>) -> TinValue {
    return vector.into_par_iter().reduce(|| TinValue::INT(0), |a, b| wrappers::sum(&a, &b));
}

pub fn parallel_mul_all(vector: Vec<TinValue>) -> TinValue {
    return vector.into_par_iter().reduce(|| TinValue::INT(1), |a, b| wrappers::mul(&a, &b));
}

pub fn parallel_max(vector: Vec<TinValue>) -> TinValue {
    return vector.into_par_iter().max_by(|a, b| {
        if wrappers::lt(&a, &b) == TinValue::INT(1){
            return std::cmp::Ordering::Less;
        }

        if a == b {
            return std::cmp::Ordering::Equal;
        }

        return std::cmp::Ordering::Greater;
        
    }).unwrap();
}

pub fn parallel_min(vector: Vec<TinValue>) -> TinValue {
    return vector.into_par_iter().min_by(|a, b| {
        if wrappers::lt(&a, &b) == TinValue::INT(1){
            return std::cmp::Ordering::Less;
        }

        if a == b {
            return std::cmp::Ordering::Equal;
        }

        return std::cmp::Ordering::Greater;
        
    }).unwrap();
}

pub fn parallel_sort_asc(vector: &mut Vec<TinValue>){
    vector.par_sort_by(|a, b| {
        if wrappers::lt(&a, &b) == TinValue::INT(1){
            return std::cmp::Ordering::Less;
        }

        if a == b {
            return std::cmp::Ordering::Equal;
        }

        return std::cmp::Ordering::Greater;
    });
}

pub fn parallel_sort_idx_asc(vector: &mut Vec<(usize, &TinValue)>) {
    vector.par_sort_by(|a, b| {
        if wrappers::lt(&a.1, &b.1) == TinValue::INT(1){
            return std::cmp::Ordering::Less;
        }

        if a.1 == b.1 {
            return std::cmp::Ordering::Equal;
        }

        return std::cmp::Ordering::Greater;
    });
}

pub fn parallel_sort_desc(vector: &mut Vec<TinValue>) {
    vector.par_sort_by(|a, b| {
        if wrappers::gt(&a, &b) == TinValue::INT(1){
            return std::cmp::Ordering::Less;
        }

        if a == b {
            return std::cmp::Ordering::Equal;
        }

        return std::cmp::Ordering::Greater;
    });
}

pub fn parallel_sort_idx_desc(vector: &mut Vec<(usize, &TinValue)>) {
    vector.par_sort_by(|a, b| {
        if wrappers::gt(&a.1, &b.1) == TinValue::INT(1){
            return std::cmp::Ordering::Less;
        }

        if a.1 == b.1 {
            return std::cmp::Ordering::Equal;
        }

        return std::cmp::Ordering::Greater;
    });
}