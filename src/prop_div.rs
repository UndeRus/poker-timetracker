pub fn proportional_int_div(numerator: u64, denominator_coeffs: &[u64]) -> Vec<u64> {
    if denominator_coeffs.is_empty() {
        return vec![];
    }

    let normalized_denominator_coeffs = normalize_coeffs(denominator_coeffs);

    let full_coeff_sum: u64 = normalized_denominator_coeffs.iter().sum();
    let min_part = numerator / full_coeff_sum;

    let remainder = numerator % full_coeff_sum;
    let mut result: Vec<u64> = normalized_denominator_coeffs
        .iter()
        .map(|x| x * min_part)
        .collect();
    if remainder > 0 {
        let mut remainder_remains = remainder;
        while remainder_remains > 0 {
            for (v, c) in result.iter_mut().zip(normalized_denominator_coeffs.iter()) {
                if remainder_remains == 0 {
                    break;
                }
                *v += 1;
                remainder_remains -= 1;

                /*
                if c * min_part > remainder_remains {
                    *v += remainder_remains;
                    remainder_remains = 0;
                } else {
                    *v += c * min_part;
                    remainder_remains -= c * min_part;
                }
                */
            }
        }
    }
    result
}

fn normalize_coeffs(input: &[u64]) -> Vec<u64> {
    if input.is_empty() {
        return input.to_vec();
    }

    if input.len() == 1 {
        return vec![1];
    }

    let first = input[0];

    let mut full_gcd = 1;
    for i in 1..input.len() {
        full_gcd = num::integer::gcd(first, input[i]);
    }

    dbg!(full_gcd);

    if full_gcd != 1 {
        return input.iter().map(|x| x / full_gcd).collect();
    }

    input.to_vec()
}

#[cfg(test)]
mod tests {

    use super::{normalize_coeffs, proportional_int_div};

    #[test]
    fn empty() {
        let coeffs = vec![];
        let numer = 100;
        let result = proportional_int_div(numer, &coeffs);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn no_remainder() {
        let coeffs = vec![1, 1, 1];
        let numer = 3;
        let result = proportional_int_div(numer, &coeffs);

        assert_eq!(result, vec![1, 1, 1]);
    }

    #[test]
    fn single_part_remainder() {
        let coeffs = vec![1, 1, 1];
        let numer = 10;
        let result = proportional_int_div(numer, &coeffs);

        assert_eq!(result, vec![4, 3, 3]);
    }

    #[test]
    fn multiple_part_remainder_equal() {
        let coeffs = vec![1, 1, 1];
        let numer = 11;
        let result = proportional_int_div(numer, &coeffs);

        assert_eq!(result, vec![4, 4, 3]);
        assert_eq!(numer, result.iter().sum());
    }

    #[test]
    fn split_big_numbers() {
        let coeffs = vec![1, 1, 1, 1, 5];
        let numer = 160;

        let result = proportional_int_div(numer, &coeffs);

        dbg!(result);
    }

    #[test]
    fn normalize_success() {
        let coeffs = vec![5, 5, 5];
        let normalized = normalize_coeffs(&coeffs);
        assert_eq!(normalized, vec![1, 1, 1]);
        assert_eq!(normalize_coeffs(&vec![2, 4, 6]), vec![1, 2, 3]);
        assert_eq!(normalize_coeffs(&vec![4, 3, 9]), vec![4, 3, 9]);
        assert_eq!(normalize_coeffs(&vec![6, 3, 9]), vec![2, 1, 3]);
    }
}
