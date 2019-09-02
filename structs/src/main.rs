fn main() {
    let user1 = User {
        username: String::from("someone"),
        email: String::from("someone@hotmail.com"),
        sign_in_count : 0,
        active: true
    };

    println!("Username is {}", user1.username);

    //Won't work unless the whole instance is mutable
    //user1.username = String::from("Some One");
    //println!("Username is {}", user1.username);

    //Makes the whole instance mutable 
    let mut user2 = User {
        username: String::from("someone else"),
        email: String::from("someone@hotmail.com"),
        sign_in_count : 0,
        active: true
    };
    
    println!("Username is {}", user2.username);
    user2.username = String::from("Someone Else");

    println!("Username is {}", user2.username);

    let built_user  = build_user(String::from("Elliot"), String::from("hurdissej@me.com"));
    println!("Username is {}", built_user.username);

    let inherited_user = User {
        username: String::from("Ellitot"),
        email: String::from("Ellitot@email.com"),
        ..built_user // inherit rest of params from other user
    };
    
    println!("Username is {}", inherited_user.username);    

}

fn build_user(username : String, email : String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct User {
    username: String,
    email: String, 
    sign_in_count: u64,
    active: bool,
}

struct Point(i32, i32, i32);