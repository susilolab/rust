use std::collections::HashSet;
use std::hash::Hash;

fn remove_dup_element_sorting<T: Ord>(elements: &mut Vec<T>) {
    elements.sort_unstable();
    elements.dedup();
}

fn main() {
    let mut sample = vec![0, 0, 1, 1, 2, 3, 2];
    println!("before: {:?}", sample);
    
    remove_dup_element_sorting(&mut sample);
    println!("after: {:?}", sample);
}
