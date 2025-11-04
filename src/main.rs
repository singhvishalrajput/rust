struct User {
    active : bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rect {
    width : i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> i32 {
        return 2 * (self.width + self.height);
    }
}


fn main() {
    let user1 = User{
        active : true,
        username : String::from("Vishal"),
        email: String::from("singh287686@gmail@gmail.com"),
        sign_in_count : 1,
    };

    println!("Status : active : {}, name : {}", user1.active, user1.username);


    let rect = Rect{
        width : 20,
        height : 15,
    };

    println!("The Area of rectangle is {}", rect.area());
    println!("The Perimeter of rectangle is {}", rect.perimeter());

}
