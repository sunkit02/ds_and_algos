pub fn linear_search<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
    for (i, e) in slice.iter().enumerate() {
        if e == target {
            return Some(i);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_linear_search() {
        let nums = [0, 1, 2, 3, 4];

        assert_eq!(linear_search(&nums, &0), Some(0));
        assert_eq!(linear_search(&nums, &4), Some(4));
        assert_eq!(linear_search(&nums, &5), None);
    }
}
