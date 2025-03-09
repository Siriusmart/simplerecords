use crate::{ParseError, Pass};

#[test]
fn _1() {
    let res = Pass::clear_comment(r#"one two # three four"#).unwrap();
    assert_eq!(res.as_str(), "one two ")
}

#[test]
fn _2() {
    let res = Pass::clear_comment(
        r#"one two # three four
five six # seven eight"#,
    )
    .unwrap();
    assert_eq!(
        res.as_str(),
        r#"one two 
five six "#
    )
}

#[test]
fn _3() {
    let res = Pass::clear_comment(r#"one two/* three four */five six"#).unwrap();
    assert_eq!(res.as_str(), r#"one two five six"#)
}

#[test]
fn _4() {
    let res = Pass::clear_comment(r#"one two"/* three four */"five six"#).unwrap();
    assert_eq!(res.as_str(), r#"one two"/* three four */"five six"#)
}

#[test]
fn _5() {
    let res = Pass::clear_comment(r#"one two '# three four'"#).unwrap();
    assert_eq!(res.as_str(), "one two '# three four'")
}

#[test]
fn _6() {
    let res = Pass::clear_comment("one two/* \nthree four */five six").unwrap();
    assert_eq!(res.as_str(), "one two\nfive six")
}

#[test]
fn _7() {
    let res = Pass::clear_comment("one two#/* three\nfour */five six").unwrap();
    assert_eq!(res.as_str(), "one two\nfour */five six")
}

#[test]
fn _8() {
    let res = Pass::clear_comment("one two/* \n\n\nthree# four */five six").unwrap();
    assert_eq!(res.as_str(), "one two\n\n\nfive six")
}

#[test]
fn _9() {
    let res = Pass::clear_comment("one two/* \n\n\nthree# four */'five six");
    assert_eq!(res, Err((4, ParseError::UnclosedString)))
}

#[test]
fn a() {
    let res = Pass::clear_comment("one two/* \n\n\nthree# four *//*five six");
    assert_eq!(res, Err((4, ParseError::UnclosedMultiLineComment)))
}

#[test]
fn b() {
    let res = Pass::clear_comment("one two\\/* \nthree four */five six").unwrap();
    assert_eq!(res.as_str(), "one two\\/* \nthree four */five six")
}

#[test]
fn c() {
    let res = Pass::clear_comment("one two\\# six#seven").unwrap();
    assert_eq!(res.as_str(), "one two\\# six")
}

#[test]
fn d() {
    let res = Pass::clear_comment("one two \\'# three four").unwrap();
    assert_eq!(res.as_str(), "one two \\'")
}

#[test]
fn e() {
    let res = Pass::clear_comment("one two \\\\'# three four'").unwrap();
    assert_eq!(res.as_str(), "one two \\\\'# three four'")
}
