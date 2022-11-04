/*!
A collection of commonly used math functions within this library

Just what was said above. There are some calculations that are used in many
of the other functions of this library. Instead of reinventing the wheel,...
*/

/// Linear Interpolation function
///
/// # Returns
/// * `start` if `amount` = 0 and `stop` if `amount` = 1
pub fn lerp(start: f64, stop: f64, amount: f64) -> f64 {
    (1.0 - amount) * start + amount * stop
}

/// Sign of direction change needed to travel from one angle to another.
///
/// For angles that are 180 degrees apart from each other, both directions have
/// the same travel distance, so either direction is shortest. The value 1.0 is
/// returned in this case.
///
/// # Arguments
///
/// | Param        | Type         | Description                                |
/// | ------------ | ------------ | ------------------------------------------ |
/// | from         | f64          | The angle travel starts from, in degrees   |
/// | to           | f64          | The angle travel ends at, in degrees.      |
///
/// # Returns
/// * -1 if decreasing from leads to the shortest travel distance,
/// * 1 if increasing from leadsto the shortest travel distance.
pub fn rotation_direction(from: f64, to: f64) -> f64 {
    let increasing_difference = sanitize_degrees_double(to - from);
    if increasing_difference <= 180.0 {
        1.0
    } else {
        -1.0
    }
}

/// Distance of two points on a circle, represented using degrees.
///
/// # Arguments
///
/// | Param        | Type         | Description      |
/// | ------------ | ------------ | ---------------- |
/// | a            | f64          | First position   |
/// | b            | f64          | Second position  |
///
/// # Returns
/// * The distance between the first position and second position as plotted on
///   a circle.
pub fn difference_degrees(a: f64, b: f64) -> f64 {
    180.0 - ((a - b).abs() - 180.0).abs()
}

/// Sanitizes a degree measure as an integer.
///
/// # Arguments
///
/// | Param        | Type         | Description                           |
/// | ------------ | ------------ | ------------------------------------- |
/// | degrees      | i32          | Angle on a circle, potentially signed |
///
/// # Returns
/// * A degree measure between 0 (inclusive) and 360 (exclusive).
pub fn sanitize_degrees_int(mut degrees: i32) -> u32 {
    degrees %= 360;
    if degrees < 0 {
        degrees += 360;
    }
    degrees as u32
}

/// Sanitizes a degree measure as a floating-point number.
///
/// # Arguments
///
/// | Param        | Type         | Description       |
/// | ------------ | ------------ | ----------------- |
/// | degrees      | f64          | Angle on a circle |
///
/// # Returns
/// * A degree measure between 0.0 (inclusive) and 360.0 (exclusive).
pub fn sanitize_degrees_double(mut degrees: f64) -> f64 {
    degrees %= 360.0;
    if degrees < 0.0 {
        degrees += 360.0;
    }
    degrees
}

/// Multiplies the Matrix. Watch out, Neo.
///
/// # Arguments
///
/// | Param        | Type           | Description       |
/// | ------------ | -------------- | ----------------- |
/// | row          | [f64; 3]       |                   |
/// | matrix       | [ [f64; 3]; 3] |                   |
///
/// # Returns
/// * Some voodoo
/// @TODO: Need to understand this a little better.
pub fn matrix_multiply(row: [f64; 3], matrix: [[f64; 3]; 3]) -> [f64; 3] {
    let a = row[0] * matrix[0][0] + row[1] * matrix[0][1] + row[2] * matrix[0][2];
    let b = row[0] * matrix[1][0] + row[1] * matrix[1][1] + row[2] * matrix[1][2];
    let c = row[0] * matrix[2][0] + row[1] * matrix[2][1] + row[2] * matrix[2][2];
    [a, b, c]
}

#[cfg(test)]
mod tests {
    use crate::utils::math::{
        difference_degrees, lerp, matrix_multiply, rotation_direction, sanitize_degrees_double,
        sanitize_degrees_int,
    };

    #[test]
    fn validate_lerp() {
        let lerp_val = lerp(12.34567, 34.5678, 1.234);
        assert_eq!(lerp_val, 39.76777842);
    }

    #[test]
    fn validate_rotation_direction() {
        let pos_rotation = rotation_direction(12.34567, 160.99876);
        let neg_rotation = rotation_direction(160.99876, 12.34567);
        assert_eq!(pos_rotation, 1.0);
        assert_eq!(neg_rotation, -1.0);
    }

    #[test]
    fn validate_difference_degrees() {
        let diff = difference_degrees(12.34567, 112.34567);
        assert_eq!(diff, 100.0);
    }

    #[test]
    fn validate_sanitize_degrees_int_pos() {
        let sanitized_int = sanitize_degrees_int(15);
        assert_eq!(sanitized_int, 15);
    }

    #[test]
    fn validate_sanitize_degrees_int_neg() {
        let sanitized_int = sanitize_degrees_int(-96);
        assert_eq!(sanitized_int, 264);
    }

    #[test]
    fn validate_sanitize_degrees_double_pos() {
        let sanitized_dbl = sanitize_degrees_double(15.1234);
        assert_eq!(sanitized_dbl, 15.1234);
    }

    #[test]
    fn validate_sanitize_degrees_double_neg() {
        let sanitized_dbl = sanitize_degrees_double(-96.3456);
        assert_eq!(sanitized_dbl, 263.6544);
    }

    #[test]
    fn validate_matrix_multiply() {
        let row = [1.25, 2.50, 3.75];
        let matrix = [[1.0, 1.0, 1.0], [2.0, 2.0, 2.0], [3.0, 3.0, 3.0]];
        let multiplied = matrix_multiply(row, matrix);
        assert_eq!(multiplied[0], 7.5);
        assert_eq!(multiplied[1], 15.0);
        assert_eq!(multiplied[2], 22.5);
    }
}
