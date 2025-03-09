use crate::Pass;

#[test]
fn _1() {
    assert_eq!(Pass::split_col("abcdef"), None);
}

#[test]
fn _2() {
    assert_eq!(
        Pass::split_col("abcdef:"),
        Some(("abcdef".to_string(), "".to_string()))
    );
}

#[test]
fn _3() {
    assert_eq!(
        Pass::split_col("abcdef:ghi"),
        Some(("abcdef".to_string(), "ghi".to_string()))
    );
}

#[test]
fn _4() {
    assert_eq!(Pass::split_col(":"), Some(("".to_string(), "".to_string())));
}

#[test]
fn _5() {
    assert_eq!(
        Pass::split_col("abcd:ef:ghi"),
        Some(("abcd".to_string(), "ef:ghi".to_string()))
    );
}

#[test]
fn _6() {
    assert_eq!(
        Pass::split_col("abc'd:ef':ghi"),
        Some(("abc'd:ef'".to_string(), "ghi".to_string()))
    );
}

#[test]
fn _7() {
    assert_eq!(
        Pass::split_col("abc\\'d:ef\\':ghi"),
        Some(("abc\\'d".to_string(), "ef\\':ghi".to_string()))
    );
}

#[test]
fn _8() {
    assert_eq!(Pass::split_col("abcdef\\:ghi"), None);
}

#[test]
fn _9() {
    assert_eq!(
        Pass::split_col("abcdef\\\\:ghi"),
        Some(("abcdef\\\\".to_string(), "ghi".to_string()))
    );
}
