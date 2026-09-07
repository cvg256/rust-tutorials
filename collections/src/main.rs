fn main1() {
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 10;
    }

    println!("After update");

    for i in &v {
        println!("{i}");
    }


}

fn main2() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}