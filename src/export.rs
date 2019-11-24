#[no_mangle]
pub extern fn sp(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern fn nothing() {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(sp(1, 2), 3);
    }
}

