fn linear_search(vector: Vec<i32>, needle: i32) -> bool {
    unimplemented!();
}

fn main() {
    let n = vec!(1,2,3,4,5);
    linear_search(n, 5);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let n = vec!(1,2,3,4,5);
        assert_eq!(
            linear_search(n, 5),
            true
        );
        let n = vec!(1,2,3,4,5);
        assert_eq!(
            linear_search(n, 6),
            false
        );
    }
}
