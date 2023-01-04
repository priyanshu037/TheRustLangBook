struct User{
    username:String,
    email:String,
    active:bool,
    sign_in_count:u64,
}
fn main(){
    let mut user1 = User{
        username:String::from("user123"),
        email:String::from("someone@gmail.com"),
        active:true,
        sign_in_count:1,
    };
    println!("{}",user1.username); // printing the username using . notation
    // changing the value of the instanance of the User
    user1.email = String::from("another@gmail.com");
}