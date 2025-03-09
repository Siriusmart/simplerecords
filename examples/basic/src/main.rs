use simplerecords::{Document, Field, Filter};

fn main() {
    // parses rules at "master.rules"
    let doc = Document::load("master").unwrap();

    // create a filter for the rule "whitelist", "aflice", *, *
    let filter1 = Filter::new(
        "whitelist".to_string(),
        &[Some(Field::IStr("alice".to_string())), None, None],
    );
    // find the first match
    let found1 = doc.find_one(filter1.clone()).unwrap();
    println!("[Filter={filter1}]");
    println!("{}", found1.unwrap());

    println!("");

    // create a filter for the rule "whitelist", *, 127.0.0.1, *
    let filter_multi = Filter::new(
        "whitelist".to_string(),
        &[None, Some(Field::IStr("127.0.0.1".to_string())), None],
    );
    // find all matches
    let found_multi = doc.find(filter_multi.clone()).unwrap();
    println!("[Filter={filter_multi}]");
    for found in found_multi {
        println!("{found}");
    }
}
