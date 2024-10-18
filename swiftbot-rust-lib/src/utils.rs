/// Clamps a value between a minimum and maximum value.
///
/// # Arguments
///
/// * `value` - The value to clamp.
/// * `min` - The minimum value.
/// * `max` - The maximum value.
///
/// # Returns
///
/// The clamped value.
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
