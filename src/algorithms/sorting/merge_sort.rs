pub fn merge_sort<T>(slice: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone,
{
    if slice.len() <= 1 {
        return Vec::from(slice);
    }
    let mid = slice.len() / 2;

    let (left, right) = slice.split_at(mid);
    let left = merge_sort(left);
    let right = merge_sort(right);

    merge(&left, &right)
}

fn merge<T>(left: &Vec<T>, right: &Vec<T>) -> Vec<T>
where
    T: PartialOrd + Clone,
{
    let left_len = left.len();
    let right_len = right.len();

    let mut merged = Vec::with_capacity(left_len + right_len);

    let mut l = 0;
    let mut r = 0;

    while l < left_len && r < right_len {
        if left[l] < right[r] {
            merged.push(left[l].clone());
            l += 1;
        } else {
            merged.push(right[r].clone());
            r += 1;
        }
    }

    while l < left_len {
        merged.push(left[l].clone());
        l += 1;
    }

    while r < right_len {
        merged.push(right[r].clone());
        r += 1;
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_sort() {
        let mut nums = [3, 1, 2, 4, 6, 5];

        assert_eq!(merge_sort(&mut nums[..]), [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_when_sorted() {
        let mut nums = [1, 2, 3, 4, 5, 6];

        assert_eq!(merge_sort(&mut nums[..]), [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_when_in_reverse_order() {
        let mut nums = [6, 5, 4, 3, 2, 1];

        assert_eq!(merge_sort(&mut nums[..]), [1, 2, 3, 4, 5, 6]);
    }
}
