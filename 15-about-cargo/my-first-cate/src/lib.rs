/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Adds left and right number.
///
/// # Examples
///
/// ```
/// let answer = add(5, 6);
///
/// assert_eq!(11, answer);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_add_on_work() {
        let arg = 5;
        let answer = add_one(arg);
        assert_eq!(answer, 6);
    }
}
