pub fn cls(){
    print!("\x1B[2J\x1B[1;1H");
}

pub fn all_equal<T: PartialEq>(vec: &[T]) -> bool {
    if let Some(first) = vec.first() {
        vec.iter().all(|x| x == first)
    } else {
        true 
    }
}

