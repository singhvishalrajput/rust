fn main() {

    let s1 = String::from("hello");

    println!("{}", s1);

    let s2 = s1;

    // println!("{}", s1);  cannot be compiled coz ownership moved to s2

    println!("{}", s2);

    let my_string = String::from("Initial String");

    takes_ownership(my_string);

    // println!(my_string);  this will cause a compile error because ownership has been moved.

}

fn takes_ownership (some_string: String) {
    println!("{}", some_string); // some_string now owns the data
}