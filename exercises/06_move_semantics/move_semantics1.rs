// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("vec1 has length {:} content `{:?}`", vec1.len(), vec1);

    vec1.push(88);

    println!("vec1 has length {:} content `{:?}`", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
