use std::vec::Vec;

#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
    max_size: usize
}

impl<T> Stack<T> {
    pub fn new(max_size: usize) -> Stack<T> {
        Stack {
            stack: Vec::new(),
            max_size: max_size
        }
    }

    pub fn push(&mut self, elem: T) -> Result<(), String> {
        if self.stack.len() == self.max_size {
            return Err(String::from("push error: max size reached"));
        }
        self.stack.push(elem);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn last(&self) -> Option<&T> {
        self.stack.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.stack.last_mut()
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }

    pub fn length(&self) -> usize {
        self.stack.len()
    }

    pub fn swap(&mut self, a: usize, b: usize) -> Result<(), String> {
        if a >= self.stack.len() {
            return Err(String::from("swap error: first index out of range"));
        }
        if b >= self.stack.len() {
            return Err(String::from("swap error: second index out of range"));
        }
        self.stack.swap(a, b);
        Ok(())
    }

    pub fn remove(&mut self, idx: usize) -> Result<(), String> {
        if idx >= self.stack.len() {
            return Err(String::from("remove error: index out of range"));
        }
        self.stack.remove(idx);
        Ok(())
    }
}
