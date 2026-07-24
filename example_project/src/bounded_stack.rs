pub struct BoundedStack<T, const N: usize> {
    stackpointer: usize,
    data: [Option<T>; N],
}

impl<T, const N: usize> BoundedStack<T, N> {
    pub fn new() -> Self {
        Self {
            stackpointer: 0,
            data: [const { None }; N],
        }
    }

    pub fn push(&mut self, t: T) {
        self.data[self.stackpointer] = Some(t);
        self.stackpointer += 1;
    }

    pub fn pop(&mut self) -> T {
        self.stackpointer -= 1;
        std::mem::replace(&mut self.data[self.stackpointer], None).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_push() {
        let mut stack: BoundedStack<_, 30> = BoundedStack::new();
        stack.push(5);
        let val = stack.pop();
        assert_eq!(val, 5, "same element should come back");
    }
}
