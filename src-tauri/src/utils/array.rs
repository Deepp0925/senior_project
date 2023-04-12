use std::mem::MaybeUninit;

/// This function creates an array of given size and fills it with the given value's default value.
/// # Warning
/// Do not use this function if you can derive 'copy' trait for the given type.
/// # Arguments
/// * `size` - the size of the array
/// # Example
/// ```
/// let array = create_array<usize, 5>();
/// assert_eq!(array, [0, 0, 0, 0, 0]);
/// ```
pub fn create_array<T: Default, const N: usize>() -> [T; N] {
    let mut array = MaybeUninit::<[T; N]>::uninit();
    // iterate over the array and fill it with default values
    for i in 0..N {
        unsafe {
            let elem = &mut (*array.as_mut_ptr())[i];
            *elem = T::default();
        };
    }

    unsafe { array.assume_init() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_array() {
        let array = create_array::<usize, 5>();
        assert_eq!(array, [0; 5]);

        let array = create_array::<Option<()>, 5>();
        assert_eq!(array, [None; 5]);
    }
}
