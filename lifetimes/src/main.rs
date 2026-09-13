use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn longest_with_announcement<'a, T>(
    x: &'a str, 
    y: &'a str, 
    announcement: T) -> &'a str
where 
    T:Display
{
    println!("Announcement! {announcement}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest_with_announcement(&string1, &string2, "Let's do it");
    println!("Longest is {result}");
}