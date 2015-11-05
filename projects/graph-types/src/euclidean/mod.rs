pub trait EuclideanDistance<T, Rhs = Self> {
    fn euclidean_distance(&self, rhs: &Rhs) -> T;
    /// It is especially suitable when only the length needs to be compared
    ///
    /// a square root operation is generally 14 instruction cycles
    ///
    /// and the square root cannot be used in some spaces, like Z^n
    fn euclidean_squared(&self, rhs: &Rhs) -> T;
}