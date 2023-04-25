#### 剑指 Offer 10- I. 斐波那契数列

```rust
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut vec = Vec::new();
        for i in 0..(n + 1) {
            match i {
                0 | 1 => vec.push(i),
                _ => vec.push((vec[(i - 1) as usize] + vec[(i - 2) as usize]) % 1000000007)
            }
        }
        vec[n as usize]
    }
}
```