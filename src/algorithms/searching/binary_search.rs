pub fn binary_search_recur<T>(slice: &[T], target: &T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    fn search<T>(slice: &[T], target: &T, lo: usize, hi: usize) -> Option<usize>
    where
        T: PartialEq + PartialOrd,
    {
        if lo > hi {
            return None;
        }
        // To avoid integer overflow when hi and lo are extremely large
        let mid = lo + ((hi - lo) / 2);

        if slice[mid] == *target {
            Some(mid)
        } else if slice[mid] < *target {
            search(slice, target, mid + 1, hi)
        } else {
            // If hi - 1 < 0 (subtract oveflow) then the target is not in the slice
            search(slice, target, lo, mid.checked_sub(1)?)
        }
    }

    search(slice, target, 0, slice.len() - 1)
}

pub fn binary_search_iter<T>(slice: &[T], target: &T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    let mut lo = 0;
    let mut hi = slice.len() - 1;

    while lo <= hi {
        let mid = lo + ((hi - lo) / 2);

        if slice[mid] == *target {
            return Some(mid);
        }

        if slice[mid] < *target {
            lo = mid + 1;
        } else {
            // If hi - 1 < 0 (subtract oveflow) then the target is not in the slice
            hi = mid.checked_sub(1)?;
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_binary_search_recur() {
        let nums = [0, 1, 2, 3, 4];

        assert_eq!(binary_search_recur(&nums, &0), Some(0));
        assert_eq!(binary_search_recur(&nums, &2), Some(2));
        assert_eq!(binary_search_recur(&nums, &4), Some(4));

        assert_eq!(binary_search_recur(&nums, &5), None);
        assert_eq!(binary_search_recur(&nums, &-1), None);
    }

    #[test]
    fn can_binary_search_iter() {
        let nums = [0, 1, 2, 3, 4];

        assert_eq!(binary_search_iter(&nums, &0), Some(0));
        assert_eq!(binary_search_iter(&nums, &2), Some(2));
        assert_eq!(binary_search_iter(&nums, &4), Some(4));

        assert_eq!(binary_search_iter(&nums, &5), None);
        assert_eq!(binary_search_iter(&nums, &-1), None);
    }
}
