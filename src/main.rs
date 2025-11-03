fn main() {

    let s1 = String::from("hello");

    println!("{}", s1);

    let s2 = s1;

    // println!("{}", s1);  cannot be compiled coz ownership moved to s2

    println!("{}", s2);

    let mut my_string = String::from("Initial String");

    my_string = takes_ownership(my_string);

    println!("{}",my_string);  

}

fn takes_ownership (some_string: String) -> String {
    println!("{}", some_string); // some_string now owns the data

    return some_string;
}