use crate::types::Type;

#[derive(Debug, Clone)]
pub struct JvmState {
    pub stack: Vec<Type>,
    pub local_variabiles: Vec<Type>,
}

impl JvmState {
    pub fn new(stack_s: u16, local_variabile_s: u16, args: Vec<Type>) -> Self {
        let mut args = args;

        let mut stack = Vec::with_capacity(stack_s as usize);
        // for _ in 0..stack_s {
        //     stack.push(Type::None)
        // }
        let mut local_variabiles = Vec::with_capacity(local_variabile_s as usize);
        for _ in 0..local_variabile_s {
            local_variabiles.push(Type::None)
        }
        for i in 0..args.len() {
            local_variabiles[i] = args.pop().unwrap()
        }

        Self {
            stack,
            local_variabiles,
        }
    }
}
