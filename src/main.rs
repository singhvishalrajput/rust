fn main() {

    let s1 = String::from("hello");

    println!("{}", s1);

    let s2 = s1;

    // println!("{}", s1);  cannot be compiled coz ownership moved to s2

    println!("{}", s2);

}