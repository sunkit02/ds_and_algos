pub fn bubble_sort<T>(slice: &mut [T])
where
    T: PartialOrd,
{
    let len = slice.len();
    for i in 0..(len - 1) {
        for j in 0..(len - i - 1) {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }
}

pub fn bubble_sort_by<T, F>(slice: &mut [T], is_greater: F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    let len = slice.len();
    for i in 0..(len - 1) {
        for j in 0..(len - i - 1) {
            if is_greater(&slice[j], &slice[j + 1]) {
                slice.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_sort() {
        let mut nums = [3, 1, 2, 4, 6, 5];
        bubble_sort(&mut nums[..]);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_when_sorted() {
        let mut nums = [1, 2, 3, 4, 5, 6];
        bubble_sort(&mut nums[..]);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_when_in_reverse_order() {
        let mut nums = [6, 5, 4, 3, 2, 1];
        bubble_sort(&mut nums[..]);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_by() {
        let mut nums = [3, 1, 2, 4, 6, 5];
        bubble_sort_by(&mut nums[..], |a, b| a > b);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_by_when_sorted() {
        let mut nums = [1, 2, 3, 4, 5, 6];
        bubble_sort_by(&mut nums[..], |a, b| a > b);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_by_when_in_reverse_order() {
        let mut nums = [6, 5, 4, 3, 2, 1];
        bubble_sort_by(&mut nums[..], |a, b| a > b);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }
}
