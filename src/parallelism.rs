use rayon::prelude::*;

use crate::interpreter::*;
use crate::wrappers;

pub fn parallel_sum_all(vector: Vec<TinValue>) -> TinValue {
    return vector.into_par_iter().reduce(|| TinValue::INT(0), |a, b| wrappers::sum(&a, &b));
}