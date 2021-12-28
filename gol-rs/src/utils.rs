pub fn to_coordinate_1d(x: i32, y: i32, width: u32) -> i32 {
    return (width as i32) * y + x;
}

pub fn to_coordinate_2d(i: i32, width: u32) -> (i32, i32) {
    return (i % (width as i32), i / (width as i32));
}

pub fn clamp<T: std::cmp::PartialOrd<T>>(input: T, min: T, max: T) -> T {
    return if input > max {
        input
    } else if input < min {
        min
    } else {
        input
    }
}