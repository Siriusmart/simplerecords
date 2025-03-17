use simplerecords::*;

fn main() {
    let doc = Options::default()
        // define data types
        .with("scope types")
        .with("user : ustr u8")
        .with("perm : ustr istr bool")
        // specify files to load from
        .with("scope imports")
        .with("include users")
        .with("include permissions")
        .open()
        .unwrap();

    dbg!(doc);
}
