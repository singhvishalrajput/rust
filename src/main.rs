fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;

    println!("{}", s1);
    println!("{}", s2);

    let my_string = String::from("initial string");
    takes_ownership(&my_string);

    println!("{}", my_string);

    // Doing hanky-panky while borrowing

    let mut str = String::from("Hello");

    update_fn(&mut str);

    println!("{}", str);


}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn update_fn(s: &mut String) {
    s.push_str(" World")
}