fn main() {

    let s1 = String::from("hello");

    println!("{}", s1);

    let s2 = s1;

    // println!("{}", s1);  cannot be compiled coz ownership moved to s2

    println!("{}", s2);

    let mut my_string = String::from("Initial String");

    my_string = takes_ownership(my_string);

    println!("{}",my_string);  
  
    stack_fn();
    heap_fn();
    update_string();
}

fn takes_ownership (some_string: String) -> String {
    println!("{}", some_string); // some_string now owns the data

    return some_string;
    stack_fn();
    heap_fn();
    update_string();
}

fn stack_fn(){
    let a = 10;
    let b = 20;
    let c = a+b;

    println!("Stack Function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn(){
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{}{}", s1, s2);

    println!("Heap function: Combined String is '{}'", combined)
}

fn update_string() {
    //start with the base string on the heap
    let mut s = String::from("Initial String");
    println!("Before update:  {}", s);
    println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    for i in 0..10 {
        s.push_str(" and some additional text");
        println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr())
    }
}