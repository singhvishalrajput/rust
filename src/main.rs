fn main() {
    let x: i32 = 4;
    let y: i32 = 10;
    let z: u32 = 300;

    println!("x: {}, y: {}, z: {}", x, y, z);

    let mut p: i32 = 10; 

    for i in 0..10 {
        p = p + 100;
        println!("Iteration {} -> p: {}", i, p);
    }

    let is_male: bool  = true;
    let is_above_18: bool = true;

    if is_male {
        print!("You are a male");
    }else{
        print!("You are not a male.");
    }

    if is_male && is_above_18 {
        print!("You are legal male");
    }

    let ax: &str = "Vishal Singh";

    print!("{}", ax);

    for i in 0..100 {
        
    }

}
