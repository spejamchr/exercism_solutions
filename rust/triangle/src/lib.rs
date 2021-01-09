use std::ops::Add;

pub trait Triangleable: Add<Output = Self> + Copy + PartialOrd {}
impl<T> Triangleable for T where T: Add<Output = T> + Copy + PartialOrd {}

pub struct Triangle {
    equal_count: usize,
}

fn calc_equal_count<T: Triangleable>(a: T, b: T, c: T) -> usize {
    [a == b, a == c, b == c].iter().filter(|a| **a).count()
}

impl Triangle {
    pub fn build<T: Triangleable>([a, b, c]: [T; 3]) -> Option<Self> {
        if a + b > c && b + c > a && a + c > b {
            let equal_count = calc_equal_count(a, b, c);
            return Some(Self { equal_count });
        }
        None
    }

    pub fn is_equilateral(&self) -> bool {
        self.equal_count == 3
    }

    pub fn is_scalene(&self) -> bool {
        self.equal_count == 0
    }

    pub fn is_isosceles(&self) -> bool {
        self.equal_count == 1
    }
}
