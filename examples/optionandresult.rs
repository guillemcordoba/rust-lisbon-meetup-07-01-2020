// An integer division that doesn't `panic!`
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in a `Some` variant
        Some(dividend / divisor)
    }
}

pub fn div(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        // This operation would `fail`, instead let's return the reason of
        // the failure wrapped in `Err`
        Err(String::from("hi"))
    } else {
        // This operation is valid, return the result wrapped in `Ok`
        Ok(x / y)
    }
}
