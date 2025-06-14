macro_rules! my_macro {
    ($first:expr $(, $rest:expr)*) => {
        ($first(), $($rest(),)*)
    };
}

fn foo() -> Vec<u8> {
    vec![1, 2, 3]
}

fn bar() -> i32 {
    1000
}

fn baz() -> String {
    String::from("string")
}

fn main() {
    let (foo_result, bar_result, baz_result) = my_macro!(foo, bar, baz);
    assert_eq!(foo_result, [1, 2, 3]);
    assert_eq!(bar_result, 1000);
    assert_eq!(baz_result, "string");
}
