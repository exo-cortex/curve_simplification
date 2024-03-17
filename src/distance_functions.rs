#[allow(dead_code)]
pub fn distance_point_to_line_2d(a: &[f64; 2], b: &[f64; 2], p: &[f64; 2]) -> f64 {
    distance_point_to_line_2d_squared(a, b, p).sqrt()
}

#[allow(dead_code)]
pub fn distance_point_to_line_2d_squared(a: &[f64; 2], b: &[f64; 2], p: &[f64; 2]) -> f64 {
    ((b[0] - a[0]) * (a[1] - p[1]) - (b[1] - a[1]) * (a[0] - p[0])).powi(2)
        / ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2))
}

#[allow(dead_code)]
pub fn distance_point_to_line_squared<const N: usize>(
    a: &[f64; N],
    b: &[f64; N],
    point: &[f64; N],
) -> f64 {
    let paba: f64 = point
        .iter()
        .zip(a)
        .zip(b)
        .map(|((p, a), b)| (p - a) * (b - a))
        .sum();
    let baba: f64 = a.iter().zip(b).map(|(a, b)| (b - a) * (b - a)).sum();
    let t = paba / baba;
    let squared_distance = a
        .iter()
        .zip(b)
        .zip(point)
        .map(|((a, b), p)| ((p - a) - t * (b - a)).powi(2))
        .sum::<f64>();
    squared_distance
}

#[cfg(test)]
mod test {
    use super::*;

    const FLOAT_CMP_DELTA: f64 = 1e-15;

    #[test]
    fn test_distance_point_to_line_squared_1() {
        let a = [0.0, 0.0];
        let b = [1.0, 0.0];
        let p = [0.5, 1.0];
        assert_eq!(distance_point_to_line_squared::<2>(&a, &b, &p), 1.0);
    }

    #[test]
    fn test_distance_point_to_line_squared_2() {
        let test_tuples = [
            ([0.0, 0.0], [1.0, 0.0], [0.5, 1.0], 1.0f64),
            ([0.0, 0.0], [1.0, 0.0], [0.5, 2.0], 2.0f64),
            ([1.0, 0.0], [0.0, 0.0], [0.5, 2.0], 2.0f64),
            ([0.0, 0.0], [1.0, 0.0], [0.5, -2.0], 2.0f64),
            ([0.0, 0.0], [-1.0, 0.0], [0.0, 1000.0], 1000.0f64),
            ([0.0, 0.0], [1.0, 0.0], [0.4, 1000.0], 1000.0f64),
            ([0.0, 0.0], [1.0, 0.0], [0.5, 3.0], 3.0f64),
            ([0.0, 0.0], [1.0, 1.0], [1.0, 0.0], 2.0f64.sqrt() / 2.0),
            ([0.0, 0.0], [2.0, 2.0], [2.0, 0.0], 2.0f64.sqrt()),
        ];

        for (i, (a, b, p, expected)) in test_tuples.iter().enumerate() {
            let actual = distance_point_to_line_squared(a, b, p);
            let difference = (actual - expected.powi(2)).abs();
            assert!(
                difference < FLOAT_CMP_DELTA,
                "test tuple #{}: {:?}, {:?}, {:?}, expected: {}, actual: {}, difference: {}",
                i + 1,
                a,
                b,
                p,
                expected,
                actual,
                difference
            );
        }
    }

    #[test]
    fn test_distance_point_to_line_squared_3() {
        let test_tuples = [
            ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 1.0, 0.0], 1.0f64),
            ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 2.0, 0.0], 2.0f64),
            ([1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.5, 2.0, 0.0], 2.0f64),
            ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 3.0, 0.0], 3.0f64),
        ];

        for (i, (a, b, p, expected)) in test_tuples.iter().enumerate() {
            let actual = distance_point_to_line_squared(a, b, p);
            let difference = (actual - expected.powi(2)).abs();
            assert!(
                difference < FLOAT_CMP_DELTA,
                "test tuple #{}: {:?}, {:?}, {:?}, expected: {}, actual: {}, difference: {}",
                i + 1,
                a,
                b,
                p,
                expected,
                actual,
                difference
            );
        }
    }
}
