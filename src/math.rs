// Greatest Common Divisor
pub fn gcd(a: i32, b: i32) -> i32 {
    if b != 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

// Least Common Multiple
pub fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}