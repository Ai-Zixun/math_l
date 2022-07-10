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

// Factorial: n!
pub fn factorial(n: i32) -> i32 {
    (1..=n).product()
}

// The number of permutation: k-permutations of n
//  - n: a set size of n
//  - k: taken k of elements from the set
// nPk = n! / (n - k)!
pub fn perm(n: i32, k: i32) -> i32 {
    (n - k + 1..=n).product()
}

// The number of combination: k-combination of n
//  - n: a set size of n
//  - k: taken k of elements from the set
// nCk = n! / ((n - k)! * k!)
pub fn comb(n: i32, k: i32) -> i32 {
    if k > n {
        0
    } else {
        (1..=k).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}
