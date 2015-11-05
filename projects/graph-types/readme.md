- Calculate the number of isomers of alkanes

```rust
#[test]
fn count() {
    let cache = SymmetricGroupCache::default();
    for c in 1..=10 {
        println!("{:>2}: {}", c, cache.a000602(c));
    }
}
```