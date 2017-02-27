pub fn fast_retain<T, F>(vec: &mut Vec<T>, mut f: F)
where F: FnMut(&T) -> bool {
    let mut i = 0;
    while i < vec.len() {
        if !f(&vec[i]) {
            vec.swap_remove(i);
        }

        i += 1;
    }
}

