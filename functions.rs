fn main(){
    // fn keyword is used to declare a new function
    // snake_case (all letters are lower case and underscores seperate words)
    another_func(5);

    y(5,'h');



}
fn another_func(x:i32){
    println!("value of x is: {x}");
}

fn y(a:i32,b:char){
    println!("value of a and b is {a},{b}")
}