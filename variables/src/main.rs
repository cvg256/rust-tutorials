fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world!");

    println!("{s1}");

    let mut s2 = s1;
    s2.push_str(", world!");

    println!("{s2}");

//    println!("{s1}");

    print_str(&mut s2);

    println!("{s2}");

}

fn print_str(str: &mut String) {
    str.push_str("[UPD]");
    println!("This is {str}");
}

