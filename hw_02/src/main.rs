fn main() {
    println!("{}", double_int32(7));
    println!("{}", double_int64(12));
    println!("{}", double_float32(2.3));
    println!("{}", double_float64(7.1));
    println!("{}", int_plus_float_to_float(11, 15.3));
    println!("{}", int_plus_float_to_int(11, 15.3));
    let x = tuple_sum((15, 32));
    println!("{}", x);
    let y = array_sum([11, 21, 31]);
    println!("{}", y);
}

fn double_int32(a: i32) -> i32 {
    a + a
}

fn double_int64(a: i32) -> i64 {
    a as i64 + a as i64
}

fn double_float32(a: f32) -> f32 {
    a + a
}

fn double_float64(a: f32) -> f64 {
    a as f64 + a as f64
}

fn int_plus_float_to_float(a: i32, b: f32) -> f64 {
    a as f64 + b as f64
}

fn int_plus_float_to_int(a: i32, b: f32) -> i64 {
    a as i64 + b as i64
}

fn tuple_sum(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn array_sum(arr: [i32; 3]) -> i32 {
    arr[0] + arr[1] + arr[2]
}
