/// Greatest common divisor (Euclidean algorithm).
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

/// Least common multiple. Non neg
pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        ((a / gcd(a, b)) * b).abs()
    }
}


/// Positive modulo: always returns a value in [0, m).
pub fn pos_mod(mut x: i64, m: i64) -> i64 {
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- gcd tests ----

    #[test]
    fn gcd_basic_cases() {
        assert_eq!(gcd(10, 5), 5);
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 13), 1); // coprime
    }

    #[test]
    fn gcd_with_zero() {
        assert_eq!(gcd(10, 0), 10);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(0, 0), 0); // conventionally returns 0
    }

    #[test]
    fn gcd_handles_negative_values() {
        assert_eq!(gcd(-10, 5), 5);
        assert_eq!(gcd(10, -5), 5);
        assert_eq!(gcd(-10, -5), 5);
    }

    // ---- lcm tests ----

    #[test]
    fn lcm_basic_cases() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(7, 5), 35);
    }

    #[test]
    fn lcm_with_zero() {
        assert_eq!(lcm(0, 10), 0);
        assert_eq!(lcm(10, 0), 0);
    }

    #[test]
    fn lcm_with_negative_values() {
        assert_eq!(lcm(-4, 6), 12);
        assert_eq!(lcm(4, -6), 12);
        assert_eq!(lcm(-4, -6), 12);
    }

    // ---- pos_mod tests ----

    #[test]
    fn pos_mod_standard_cases() {
        assert_eq!(pos_mod(10, 3), 1);
        assert_eq!(pos_mod(7, 7), 0);
        assert_eq!(pos_mod(1, 5), 1);
    }

    #[test]
    fn pos_mod_handles_negatives() {
        assert_eq!(pos_mod(-1, 5), 4);
        assert_eq!(pos_mod(-6, 5), 4); // -6 % 5 = -1
        assert_eq!(pos_mod(-11, 5), 4);
    }

    #[test]
    fn pos_mod_large_values() {
        assert_eq!(pos_mod(123456789, 97), 123456789 % 97);
        assert_eq!(pos_mod(-123456789, 97), pos_mod(-(123456789 % 97), 97));
    }
}
