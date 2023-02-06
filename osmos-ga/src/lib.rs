pub trait Fitness {
    fn fitness(&self) -> isize;
}

impl Fitness for osmos_core::cell::Cell {
    fn fitness(&self) -> isize {
        self.energy as isize
    }
}

pub fn selection() -> f32 {
    todo!()
}

pub fn crossover() {
    todo!()
}

pub fn mutation() {
    todo!()
}

pub fn evole() {
    todo!()
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test() {
    // }
}
