fn measure_time<T, U>(f: impl Fn(U) -> T, arg: U) -> (T, u128) {
    let start = std::time::Instant::now();
    (f(arg), start.elapsed().as_nanos())
}

fn sum(a: u64, b: u64) -> u64 {
    a + b
}

fn concat(a: &[&str]) -> String {
    a.join(" ")
}

fn main() {
    let (res, execution_time) = measure_time(|(a, b)| sum(a, b), (10, 20));
    assert_eq!(30, res);
    println!("sum took {execution_time:?} nanoseconds");

    let (res, execution_time) = measure_time(|strs| concat(strs), &["hello", "world"]);
    assert_eq!("hello world", res);
    println!("concat took {execution_time:?} nanoseconds");

    let (_, execution_time) = measure_time(
        |_| {
            println!("printing to stdout");
        },
        (),
    );
    println!("printing took {execution_time:?} nanoseconds");
}
