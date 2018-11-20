use std;
pub fn insertion_sort<T>(list: &mut [T])
where
    T: std::cmp::PartialOrd + std::fmt::Debug + std::cmp::Ord + std::clone::Clone,
{
    for i in 1..list.len() {
        let mut j = (i - 1) as i32;
        let elem = &(list[i]).clone();

        while j >= 0 && list[j as usize].cmp(elem) == std::cmp::Ordering::Greater {
            replace(&mut list[j as usize], &mut list[(j + 1) as usize]);
            j = j - 1;
        }
        copy(elem, &mut list[(j + 1) as usize]);
    }
}

fn copy<T>(x: &T, y: &mut T) {
    unsafe {
        std::ptr::copy(x, y, 1);
    }
}

fn replace<T>(x: *mut T, y: *mut T) {
    unsafe {
        std::ptr::copy_nonoverlapping(x, y, 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insertion_sort_test() {
        let mut abc = vec![5, 6, 7, 8, 1];
        insertion_sort(&mut abc);
        assert_eq!(abc, vec![1, 5, 6, 7, 8])
    }
}
