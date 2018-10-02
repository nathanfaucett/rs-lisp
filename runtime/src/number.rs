use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::{fmt, mem};

use num_traits::FromPrimitive;

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum Number {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    ISize(isize),
    F32(f32),
    F64(f64),
}

impl Number {
    #[inline]
    pub fn cast<T>(&self) -> T
    where
        T: FromPrimitive,
    {
        match self {
            &Number::U8(ref a) => T::from_u8(*a).unwrap(),
            &Number::U16(ref a) => T::from_u16(*a).unwrap(),
            &Number::U32(ref a) => T::from_u32(*a).unwrap(),
            &Number::U64(ref a) => T::from_u64(*a).unwrap(),
            &Number::USize(ref a) => T::from_usize(*a).unwrap(),
            &Number::I8(ref a) => T::from_i8(*a).unwrap(),
            &Number::I16(ref a) => T::from_i16(*a).unwrap(),
            &Number::I32(ref a) => T::from_i32(*a).unwrap(),
            &Number::I64(ref a) => T::from_i64(*a).unwrap(),
            &Number::ISize(ref a) => T::from_isize(*a).unwrap(),
            &Number::F32(ref a) => T::from_f32(*a).unwrap(),
            &Number::F64(ref a) => T::from_f64(*a).unwrap(),
        }
    }
}

impl fmt::Debug for Number {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Number::U8(ref a) => write!(f, "{}_u8", a),
            &Number::U16(ref a) => write!(f, "{}_u16", a),
            &Number::U32(ref a) => write!(f, "{}_u32", a),
            &Number::U64(ref a) => write!(f, "{}_u64", a),
            &Number::USize(ref a) => write!(f, "{}_usize", a),
            &Number::I8(ref a) => write!(f, "{}_i8", a),
            &Number::I16(ref a) => write!(f, "{}_i16", a),
            &Number::I32(ref a) => write!(f, "{}_i32", a),
            &Number::I64(ref a) => write!(f, "{}_i64", a),
            &Number::ISize(ref a) => write!(f, "{}_isize", a),
            &Number::F32(ref a) => write!(f, "{}_f32", a),
            &Number::F64(ref a) => write!(f, "{}_f64", a),
        }
    }
}

impl fmt::Display for Number {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Ord for Number {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            &Number::U8(ref a) => match other {
                &Number::U8(ref b) => a.cmp(b),
                _ => a.cmp(&other.cast::<u8>()),
            },
            &Number::U16(ref a) => match other {
                &Number::U16(ref b) => a.cmp(b),
                _ => a.cmp(&other.cast::<u16>()),
            },
            &Number::U32(ref a) => match other {
                &Number::U32(ref b) => a.cmp(b),
                _ => a.cmp(&other.cast::<u32>()),
            },
            &Number::U64(ref a) => match other {
                &Number::U64(ref b) => a.cmp(b),
                _ => a.cmp(&other.cast::<u64>()),
            },
            &Number::USize(ref a) => match other {
                &Number::USize(ref b) => a.cmp(b),
                _ => a.cmp(&other.cast::<usize>()),
            },
            &Number::I8(ref a) => match other {
                &Number::I8(ref b) => a.cmp(b),
                _ => a.cmp(&other.cast::<i8>()),
            },
            &Number::I16(ref a) => match other {
                &Number::I16(ref b) => a.cmp(b),
                _ => a.cmp(&other.cast::<i16>()),
            },
            &Number::I32(ref a) => match other {
                &Number::I32(ref b) => a.cmp(b),
                _ => a.cmp(&other.cast::<i32>()),
            },
            &Number::I64(ref a) => match other {
                &Number::I64(ref b) => a.cmp(b),
                _ => a.cmp(&other.cast::<i64>()),
            },
            &Number::ISize(ref a) => match other {
                &Number::ISize(ref b) => a.cmp(b),
                _ => a.cmp(&other.cast::<isize>()),
            },
            &Number::F32(ref a) => match other {
                &Number::F32(ref b) => unsafe {
                    let a = mem::transmute::<f32, i32>(*a);
                    let b = mem::transmute::<f32, i32>(*b);
                    a.cmp(&b)
                },
                _ => unsafe {
                    let a = mem::transmute::<f32, i32>(*a);
                    let b = mem::transmute::<f32, i32>(other.cast::<f32>());
                    a.cmp(&b)
                },
            },
            &Number::F64(ref a) => match other {
                &Number::F64(ref b) => unsafe {
                    let a = mem::transmute::<f64, i64>(*a);
                    let b = mem::transmute::<f64, i64>(*b);
                    a.cmp(&b)
                },
                _ => unsafe {
                    let a = mem::transmute::<f64, i64>(*a);
                    let b = mem::transmute::<f64, i64>(other.cast::<f64>());
                    a.cmp(&b)
                },
            },
        }
    }
}

impl Hash for Number {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            &Number::U8(ref a) => a.hash(state),
            &Number::U16(ref a) => a.hash(state),
            &Number::U32(ref a) => a.hash(state),
            &Number::U64(ref a) => a.hash(state),
            &Number::USize(ref a) => a.hash(state),
            &Number::I8(ref a) => a.hash(state),
            &Number::I16(ref a) => a.hash(state),
            &Number::I32(ref a) => a.hash(state),
            &Number::I64(ref a) => a.hash(state),
            &Number::ISize(ref a) => a.hash(state),
            &Number::F32(ref a) => unsafe {
                let a = mem::transmute::<f32, i32>(*a);
                a.hash(state)
            },
            &Number::F64(ref a) => unsafe {
                let a = mem::transmute::<f64, i64>(*a);
                a.hash(state)
            },
        }
    }
}

impl Eq for Number {}
