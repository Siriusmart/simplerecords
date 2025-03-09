use crate::Pass;

#[test]
fn _1() {
    assert_eq!(
        Pass::split_args("abc def ghi"),
        Ok(vec![
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string()
        ])
    )
}

#[test]
fn _2() {
    assert_eq!(
        Pass::split_args("abc de\\f ghi"),
        Ok(vec![
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string()
        ])
    )
}

#[test]
fn _3() {
    assert_eq!(
        Pass::split_args("abc def\\ ghi"),
        Err(crate::ParseError::IllegalArgument)
    )
}

#[test]
fn _4() {
    assert_eq!(
        Pass::split_args("abc 'def ghi'"),
        Ok(vec!["abc".to_string(), "def ghi".to_string(),])
    )
}

#[test]
fn _5() {
    assert_eq!(
        Pass::split_args("abc 'def\\ ghi'"),
        Ok(vec!["abc".to_string(), "def ghi".to_string(),])
    )
}

#[test]
fn _6() {
    assert_eq!(
        Pass::split_args("abc 'def\\' ghi'"),
        Ok(vec!["abc".to_string(), "def' ghi".to_string(),])
    )
}

#[test]
fn _7() {
    assert_eq!(
        Pass::split_args("abc '12 34 56' `48 ' 9'7`"),
        Ok(vec![
            "abc".to_string(),
            "12 34 56".to_string(),
            "48 ' 9'7".to_string(),
        ])
    )
}

#[test]
fn _8() {
    assert_eq!(
        Pass::split_args("    abc     '12 34 56'       `48 ' 9'7`"),
        Ok(vec![
            "abc".to_string(),
            "12 34 56".to_string(),
            "48 ' 9'7".to_string(),
        ])
    )
}
