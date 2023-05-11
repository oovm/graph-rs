use super::*;

impl Node for usize {
    type Name = String;

    fn index(&self) -> usize {
        *self
    }

    fn name(&self) -> Self::Name {
        self.to_string()
    }
}
