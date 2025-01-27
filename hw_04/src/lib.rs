pub fn get_tuple_element<T>(tuple: &mut (T, T), flag: bool) -> &mut T {
    if flag {
        &mut tuple.1
    } else {
        &mut tuple.0
    }
}

pub fn get_n_slice_element<T>(slice: &mut [T], n: usize) -> &mut T {
    if slice.len() <= n {
        println!("Index: {:?} out of bounds!", n);
    }
    &mut slice[n]
}

pub fn get_n_end_slice_element<T>(slice: &[T], n: usize) -> &T {
    if slice.len() <= n {
        println!("Index: {:?} out of bounds!", n);
    }
    &slice[slice.len() - 1 - n]
}

pub fn get_two_slices<T>(slice: &[T], n: usize) -> (&[T], &[T]) {
    if slice.len() <= n {
        println!("Index: {:?} out of bounds!", n);
    }
    (&slice[..n], &slice[n..])
}

pub fn get_four_slices<T>(slice: &[T]) -> [&[T]; 4] {
    let mut arr = [&[] as &[T]; 4];
    let c = (slice.len() as f32 / 4.).round() as usize;
    if c == 0 {
        for i in 0..slice.len() {
            arr[i] = &slice[i..=i];
        }
        return arr;
    }
    arr[0] = &slice[0..c];
    arr[1] = &slice[c..(2 * c)];
    arr[2] = &slice[(2 * c)..(3 * c)];
    arr[3] = &slice[(3 * c)..];
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tuple_element_true() {
        let mut tuple = (1, 2);
        let e = get_tuple_element(&mut tuple, true);
        assert_eq!(e, &mut 2);
    }

    #[test]
    fn get_tuple_element_false() {
        let mut tuple = (1, 2);
        let e = get_tuple_element(&mut tuple, false);
        assert_eq!(e, &mut 1);
    }

    #[test]
    fn get_first_slice_element() {
        let mut numbers = [2, 3, 5, 7, 11, 13, 17, 19];
        let e = get_n_slice_element(&mut numbers[..], 0);
        assert_eq!(e, &mut 2);
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds_slice_element() {
        let mut numbers = [2, 3, 5, 7, 11, 13, 17, 19];
        get_n_slice_element(&mut numbers[..], 10);
    }

    #[test]
    fn get_first_end_slice_element() {
        let numbers = [2, 3, 5, 7, 11, 13, 17, 19];
        let e = get_n_end_slice_element(&numbers[..], 0);
        assert_eq!(e, &19);
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds_end_slice_element() {
        let numbers = [2, 3, 5, 7, 11, 13, 17, 19];
        get_n_end_slice_element(&numbers[..], 10);
    }

    #[test]
    fn get_two_slices_ok() {
        let numbers = [2, 3, 5, 7, 11, 13, 17, 19];
        let (slice1, slice2) = get_two_slices(&numbers[..], 3);
        assert_eq!(slice1, &[2, 3, 5]);
        assert_eq!(slice2, &[7, 11, 13, 17, 19]);
    }

    #[test]
    #[should_panic]
    fn get_two_slices_out_of_bounds() {
        let numbers = [2, 3, 5, 7, 11, 13, 17, 19];
        get_two_slices(&numbers[..], 10);
    }

    #[test]
    fn get_four_slices_one() {
        let numbers = [2, 3, 5];
        let a = get_four_slices(&numbers[..]);
        assert_eq!(a[0], &[2]);
        assert_eq!(a[1], &[3]);
        assert_eq!(a[2], &[5]);
        assert_eq!(a[3], &[]);
    }

    #[test]
    fn get_four_slices_two() {
        let numbers = [2, 3, 5, 7, 11];
        let a = get_four_slices(&numbers[..]);
        assert_eq!(a[0], &[2]);
        assert_eq!(a[1], &[3]);
        assert_eq!(a[2], &[5]);
        assert_eq!(a[3], &[7, 11]);
    }

    #[test]
    fn get_four_slices_three() {
        let numbers = [2, 3, 5, 7, 11, 13, 17, 19];
        let a = get_four_slices(&numbers[..]);
        assert_eq!(a[0], &[2, 3]);
        assert_eq!(a[1], &[5, 7]);
        assert_eq!(a[2], &[11, 13]);
        assert_eq!(a[3], &[17, 19]);
    }
}
