pub fn quick_sort<T>(slice: &mut [T])
where
    T: PartialOrd,
{
    if slice.len() <= 1 {
        return;
    }

    let p = partition(slice);
    quick_sort(&mut slice[..p]);
    quick_sort(&mut slice[p..]);
}

pub fn quick_sort_by<T, F>(slice: &mut [T], mut is_less: F)
where
    T: PartialOrd,
    F: FnMut(&T, &T) -> bool + Copy,
{
    if slice.len() <= 1 {
        return;
    }

    let p = partition_by(slice, &mut is_less);
    quick_sort_by(&mut slice[..p], is_less);
    quick_sort_by(&mut slice[p..], is_less);
}

fn partition<T>(slice: &mut [T]) -> usize
where
    T: PartialOrd,
{
    // TODO: Randomly select pivot point
    let p = slice.len() - 1;

    // Move pivot point to end of slice
    slice.swap(p, slice.len() - 1);

    let mut l = 0;
    let mut r = 0;

    while r < p {
        if slice[r] < slice[p] {
            slice.swap(l, r);
            l += 1;
        }
        r += 1;
    }

    slice.swap(p, l);

    p
}

fn partition_by<T, F>(slice: &mut [T], mut is_less: F) -> usize
where
    T: PartialOrd,
    F: FnMut(&T, &T) -> bool,
{
    // TODO: Randomly select pivot point
    let p = slice.len() - 1;

    // Move pivot point to end of slice
    slice.swap(p, slice.len() - 1);

    let mut l = 0;
    let mut r = 0;

    while r < p {
        if is_less(&slice[r], &slice[p]) {
            slice.swap(l, r);
            l += 1;
        }
        r += 1;
    }

    slice.swap(p, l);

    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_sort() {
        let mut nums = [3, 1, 2, 4, 6, 5];
        quick_sort(&mut nums[..]);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_when_sorted() {
        let mut nums = [1, 2, 3, 4, 5, 6];
        quick_sort(&mut nums[..]);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_when_in_reverse_order() {
        let mut nums = [6, 5, 4, 3, 2, 1];
        quick_sort(&mut nums[..]);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_by() {
        let mut nums = [3, 1, 2, 4, 6, 5];
        quick_sort_by(&mut nums[..], |a, b| a < b);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_by_when_sorted() {
        let mut nums = [1, 2, 3, 4, 5, 6];
        quick_sort_by(&mut nums[..], |a, b| a < b);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn can_sort_by_when_in_reverse_order() {
        let mut nums = [6, 5, 4, 3, 2, 1];
        quick_sort_by(&mut nums[..], |a, b| a < b);

        assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
    }
}
