pub trait Sealed {}

impl<T> Sealed for &mut T where T: Sealed {}

/// Gets the maximum value of `usize` Array. This is not generic because we can't yet require const implementation for traits.
pub const fn get_max_value<const N: usize>(arr: [usize; N]) -> usize {
    let mut current_index = 0;
    let mut current_max = 0;

    while current_index < N {
        let current_value = arr[current_index];
        if current_value > current_max {
            current_max = current_value
        }
        current_index += 1;
    }
    current_max
}