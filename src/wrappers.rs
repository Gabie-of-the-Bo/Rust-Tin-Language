use crate::interpreter::{*};

pub fn floor(a: &TinValue) -> TinValue{
    return match a{
        TinValue::INT(n) => TinValue::INT(*n),
        TinValue::FLOAT(n) => TinValue::INT(n.floor() as i64),
        TinValue::VECTOR(v) => TinValue::VECTOR(v.iter().map(floor).collect()),

        _ => unreachable!()
    }
}

pub fn ceil(a: &TinValue) -> TinValue{
    return match a{
        TinValue::INT(n) => TinValue::INT(*n),
        TinValue::FLOAT(n) => TinValue::INT(n.ceil() as i64),
        TinValue::VECTOR(v) => TinValue::VECTOR(v.iter().map(ceil).collect()),

        _ => unreachable!()
    }
}

pub fn truthy(a: &TinValue) -> TinValue{
    return match a{
        TinValue::VECTOR(v) => TinValue::VECTOR(v.iter().map(truthy).collect()),

        _ => TinValue::INT(a.truthy() as i64)
    }
}

pub fn lt(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT((a < b) as i64),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::INT((*b > *a as f64) as i64),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::INT((*a < *b as f64) as i64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::INT((a < b) as i64),

        (TinValue::INT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| lt(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::INT(_)) => TinValue::VECTOR(b.iter().map(|v| lt(v, bb)).collect::<Vec<_>>()),

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| lt(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => TinValue::VECTOR(b.iter().map(|v| lt(v, bb)).collect::<Vec<_>>()),

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => TinValue::VECTOR(a.iter().zip(b).map(|t| lt(t.0, t.1)).collect::<Vec<_>>()),

        _ => unreachable!()
    };
}

pub fn gt(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT((a > b) as i64),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::INT((*a as f64 > *b) as i64),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::INT((*a > *b as f64) as i64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::INT((a > b) as i64),

        (TinValue::INT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| gt(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::INT(_)) => TinValue::VECTOR(b.iter().map(|v| gt(v, bb)).collect::<Vec<_>>()),

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| gt(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => TinValue::VECTOR(b.iter().map(|v| gt(v, bb)).collect::<Vec<_>>()),

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => TinValue::VECTOR(a.iter().zip(b).map(|t| gt(t.0, t.1)).collect::<Vec<_>>()),

        _ => unreachable!()
    };
}

pub fn sum(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT(a + b),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(*a as f64 + b),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::FLOAT(a + *b as f64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(a + b),

        (TinValue::INT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| sum(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::INT(_)) => TinValue::VECTOR(b.iter().map(|v| sum(v, bb)).collect::<Vec<_>>()),

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| sum(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => TinValue::VECTOR(b.iter().map(|v| sum(v, bb)).collect::<Vec<_>>()),

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => TinValue::VECTOR(a.iter().zip(b).map(|t| sum(t.0, t.1)).collect::<Vec<_>>()),

        _ => unreachable!()
    };
}

pub fn sub(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT(a - b),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(*a as f64 - b),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::FLOAT(a - *b as f64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(a - b),

        (TinValue::INT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| sub(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::INT(_)) => TinValue::VECTOR(b.iter().map(|v| sub(v, bb)).collect::<Vec<_>>()),

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| sub(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => TinValue::VECTOR(b.iter().map(|v| sub(v, bb)).collect::<Vec<_>>()),

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => TinValue::VECTOR(a.iter().zip(b).map(|t| sub(t.0, t.1)).collect::<Vec<_>>()),

        _ => unreachable!()
    };
}

pub fn mul(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT(a * b),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(*a as f64 * b),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::FLOAT(a * *b as f64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(a * b),

        (TinValue::INT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| mul(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::INT(_)) => TinValue::VECTOR(b.iter().map(|v| mul(v, bb)).collect::<Vec<_>>()),

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| mul(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => TinValue::VECTOR(b.iter().map(|v| mul(v, bb)).collect::<Vec<_>>()),

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => TinValue::VECTOR(a.iter().zip(b).map(|t| mul(t.0, t.1)).collect::<Vec<_>>()),

        _ => unreachable!()
    };
}

pub fn div(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT(a / b),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(*a as f64 / b),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::FLOAT(a / *b as f64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(a / b),

        (TinValue::INT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| div(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::INT(_)) => TinValue::VECTOR(b.iter().map(|v| div(v, bb)).collect::<Vec<_>>()),

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| div(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => TinValue::VECTOR(b.iter().map(|v| div(v, bb)).collect::<Vec<_>>()),

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => TinValue::VECTOR(a.iter().zip(b).map(|t| div(t.0, t.1)).collect::<Vec<_>>()),

        _ => unreachable!()
    };
}

pub fn modl(aa: &TinValue, bb: &TinValue) -> TinValue{
    return match (aa, bb) {
        (TinValue::INT(a), TinValue::INT(b)) => TinValue::INT(a % b),
        (TinValue::INT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(*a as f64 % b),
        (TinValue::FLOAT(a), TinValue::INT(b)) => TinValue::FLOAT(a % *b as f64),
        (TinValue::FLOAT(a), TinValue::FLOAT(b)) => TinValue::FLOAT(a % b),

        (TinValue::INT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| modl(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::INT(_)) => TinValue::VECTOR(b.iter().map(|v| modl(v, bb)).collect::<Vec<_>>()),

        (TinValue::FLOAT(_), TinValue::VECTOR(b)) => TinValue::VECTOR(b.iter().map(|v| modl(aa, v)).collect::<Vec<_>>()),
        (TinValue::VECTOR(b), TinValue::FLOAT(_)) => TinValue::VECTOR(b.iter().map(|v| modl(v, bb)).collect::<Vec<_>>()),

        (TinValue::VECTOR(a), TinValue::VECTOR(b)) => TinValue::VECTOR(a.iter().zip(b).map(|t| modl(t.0, t.1)).collect::<Vec<_>>()),

        _ => unreachable!()
    };
}