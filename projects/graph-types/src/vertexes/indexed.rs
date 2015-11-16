use super::*;

impl Node for usize {
    fn index(&self) -> usize {
        *self
    }
}
