fn matching_along_path_mut<T>(
    slice: &mut [T],               // slice to examine
    mut index: usize,              // starting index
    next: impl Fn(usize) -> usize, // calculate next index
    valid: impl Fn(&T) -> bool,    // return this thing?
) -> Option<&mut T> {
    loop {
        let x = slice.get_mut(index)?;
        if valid(x) {
            return Some(x);
        }
        index = next(index);
    }
}

fn main() {}
