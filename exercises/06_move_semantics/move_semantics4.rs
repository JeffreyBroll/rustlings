// move_semantics4.rs
// Refactor this code so that instead of passing `vec0` into the `fill_vec` function,
// the Vector gets created in the function itself and passed back to the main
// function.
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics5() {
        let mut x = 100;
        let y = &mut x;
        let z = &mut x;
        *y += 100;
        *z += 1000;
        assert_eq!(x, 1200);
    }
}
