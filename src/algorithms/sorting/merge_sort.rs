use std::mem;

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

/// WARN: Not actually zero copy and performs worse than the normal merge sort.
/// Bad, very bad. Did I say that this is bad? Yes, it's very bad.
pub fn merge_sort_zero_copy<T>(slice: &mut [T])
where
    T: PartialOrd + Clone,
{
    if slice.len() < 2 {
        return;
    } else if slice.len() == 2 {
        if slice[0] > slice[1] {
            slice.swap(0, 1);
        }

        return;
    }

    let (left, right) = slice.split_at_mut(slice.len() / 2);
    merge_sort_zero_copy(left);
    merge_sort_zero_copy(right);

    merge_zero_copy(left, right);
}

/// Start with two slices `left` and `right` that are already sorted with `l_len == (r_len + 1) || (r_len - 1)`
///
///  - Have two pointers `l` and `r` pointing to the start of the `left` and `right` slices.
///  - While `l` is less than `l_len`
///     - Compare the values at `left[l]` and `right[r]`
///     - If `left[l]` is greater than `right[r]` swap the two values.
///       - Bubble sort the value that was swapped on the right slice (keep the values on the right greater than the left and sorted)
///
fn merge_zero_copy<T>(left: &mut [T], right: &mut [T])
where
    T: PartialOrd + Clone,
{
    if left.len() == 0 || right.len() == 0 {
        return;
    }

    let left_len = left.len();
    let right_len = right.len();
    let mut l = 0;

    while l < left_len {
        if left[l] > right[0] {
            mem::swap(&mut left[l], &mut right[0]);

            // Sort swapped value
            let mut i = 0;
            while i < right_len - 1 {
                if right[i] > right[i + 1] {
                    right.swap(i, i + 1);
                }
                i += 1;
            }
        }

        l += 1;
    }
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

    #[test]
    fn can_sort_zero_copy() {
        let mut nums = [3, 1, 2, 4, 6, 5];

        assert_eq!(merge_sort(&mut nums[..]), [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_when_sorted_zero_copy() {
        let mut nums = [1, 2, 3, 4, 5, 6];

        assert_eq!(merge_sort(&mut nums[..]), [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_when_in_reverse_order_zero_copy() {
        let mut nums = [6, 5, 4, 3, 2, 1];

        assert_eq!(merge_sort(&mut nums[..]), [1, 2, 3, 4, 5, 6]);
    }
}
