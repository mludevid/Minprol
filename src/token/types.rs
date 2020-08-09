#[derive(Debug, Clone)]
pub enum Type {
    TtI32(i32),
    TtI64(i64),
    TtF32(f32),
    TtF64(f64),
    TtU32(u32),
    TtU64(u64),
}

/*
pub fn type_to_i32(token: Type) -> i32 {
    if let Type::TtI32(i) = token {
        return i;       
    } else {
        panic!("Expected TtI32");
    }
}

pub fn type_to_i64(token: Type) -> i64 {
    if let Type::TtI64(i) = token {
        return i;       
    } else {
        panic!("Expected TtI64");
    }
}

pub fn type_to_f32(token: Type) -> f32 {
    if let Type::TtF32(i) = token {
        return i;       
    } else {
        panic!("Expected TtF32");
    }
}

pub fn type_to_f64(token: Type) -> f64 {
    if let Type::TtF64(i) = token {
        return i;       
    } else {
        panic!("Expected TtF64");
    }
}
*/

pub fn add_types(op1: &Type, op2: &Type) -> Type {
    match op1 {
        Type::TtI32(i) => {
            match op2 {
                Type::TtI32(j) => Type::TtI32(i + j),
                _ => panic!("You can only add I32 to I32"),
            }
        },
        Type::TtI64(i) => {
            match op2 {
                Type::TtI64(j) => Type::TtI64(i + j),
                _ => panic!("You can only add I64 to I64"),
            }
        },
        Type::TtF32(i) => {
            match op2 {
                Type::TtF32(j) => Type::TtF32(i + j),
                _ => panic!("You can only add F32 to F32"),
            }
        },
        Type::TtF64(i) => {
            match op2 {
                Type::TtF64(j) => Type::TtF64(i + j),
                _ => panic!("You can only add F64 to F64"),
            }
        },
        Type::TtU32(i) => {
            match op2 {
                Type::TtU32(j) => Type::TtU32(i + j),
                _ => panic!("You can only add U32 to U32"),
            }
        },
        Type::TtU64(i) => {
            match op2 {
                Type::TtU64(j) => Type::TtU64(i + j),
                _ => panic!("You can only add U64 to U64"),
            }
        },
    }
}

pub fn substract_types(op1: &Type, op2: &Type) -> Type {
    match op1 {
        Type::TtI32(i) => {
            match op2 {
                Type::TtI32(j) => Type::TtI32(i - j),
                _ => panic!("You can only substract I32 to I32"),
            }
        },
        Type::TtI64(i) => {
            match op2 {
                Type::TtI64(j) => Type::TtI64(i - j),
                _ => panic!("You can only substract I64 to I64"),
            }
        },
        Type::TtF32(i) => {
            match op2 {
                Type::TtF32(j) => Type::TtF32(i - j),
                _ => panic!("You can only substract F32 to F32"),
            }
        },
        Type::TtF64(i) => {
            match op2 {
                Type::TtF64(j) => Type::TtF64(i - j),
                _ => panic!("You can only substract F64 to F64"),
            }
        },
        Type::TtU32(i) => {
            match op2 {
                Type::TtU32(j) => Type::TtU32(i - j),
                _ => panic!("You can only substract U32 to U32"),
            }
        },
        Type::TtU64(i) => {
            match op2 {
                Type::TtU64(j) => Type::TtU64(i - j),
                _ => panic!("You can only substract U64 to U64"),
            }
        },
    }
}

pub fn multiply_types(op1: &Type, op2: &Type) -> Type {
    match op1 {
        Type::TtI32(i) => {
            match op2 {
                Type::TtI32(j) => Type::TtI32(i * j),
                _ => panic!("You can only multiply I32 to I32"),
            }
        },
        Type::TtI64(i) => {
            match op2 {
                Type::TtI64(j) => Type::TtI64(i * j),
                _ => panic!("You can only multiply I64 to I64"),
            }
        },
        Type::TtF32(i) => {
            match op2 {
                Type::TtF32(j) => Type::TtF32(i * j),
                _ => panic!("You can only multiply F32 to F32"),
            }
        },
        Type::TtF64(i) => {
            match op2 {
                Type::TtF64(j) => Type::TtF64(i * j),
                _ => panic!("You can only multiply F64 to F64"),
            }
        },
        Type::TtU32(i) => {
            match op2 {
                Type::TtU32(j) => Type::TtU32(i * j),
                _ => panic!("You can only multiply U32 to U32"),
            }
        },
        Type::TtU64(i) => {
            match op2 {
                Type::TtU64(j) => Type::TtU64(i * j),
                _ => panic!("You can only multiply U64 to U64"),
            }
        },
    }
}

pub fn divide_types(op1: &Type, op2: &Type) -> Type {
    match op1 {
        Type::TtI32(i) => {
            match op2 {
                Type::TtI32(j) => Type::TtI32(i / j),
                _ => panic!("You can only divide I32 to I32"),
            }
        },
        Type::TtI64(i) => {
            match op2 {
                Type::TtI64(j) => Type::TtI64(i / j),
                _ => panic!("You can only divide I64 to I64"),
            }
        },
        Type::TtF32(i) => {
            match op2 {
                Type::TtF32(j) => Type::TtF32(i / j),
                _ => panic!("You can only divide F32 to F32"),
            }
        },
        Type::TtF64(i) => {
            match op2 {
                Type::TtF64(j) => Type::TtF64(i / j),
                _ => panic!("You can only divide F64 to F64"),
            }
        },
        Type::TtU32(i) => {
            match op2 {
                Type::TtU32(j) => Type::TtU32(i / j),
                _ => panic!("You can only divide U32 to U32"),
            }
        },
        Type::TtU64(i) => {
            match op2 {
                Type::TtU64(j) => Type::TtU64(i / j),
                _ => panic!("You can only divide U64 to U64"),
            }
        },
    }
}

pub fn negate_type(op: Type) -> Type {
    match op {
        Type::TtI32(i) => {
            Type::TtI32(-i)
        },
        Type::TtI64(i) => {
            Type::TtI64(-i)
        },
        Type::TtF32(i) => {
            Type::TtF32(-i)
        },
        Type::TtF64(i) => {
            Type::TtF64(-i)
        },
        Type::TtU32(_) => {
            panic!("You can't negate U32")
        },
        Type::TtU64(_) => {
            panic!("You can't negate U64")
        },
    }
}