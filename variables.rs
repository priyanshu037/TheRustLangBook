fn main(){
    let x = 5;
    println!("value of x is {}",x);
    // x = 6; because by default variable is immutable
    println!("value of x is {}",x); // error


    // creating a mutable variable using 'mut' keyword
    let mut a = 5;
    println!("The value of a is {}",a);
    a = 6;
    println!("The value of a is {}",a);

    // constants
    // not allowed to use 'mut' keyword
    // they are always immutable
    // declare using const keyword

    const VAR_1: i32 = 34;
    println!("{}",VAR_1);


    // shadowing

    let x = 4;
    println!("x is : {}",x);
    {
        let x = 2;
        println!("x is : {}", x)
    }
    let x = x + 1;
    println!("x is : {}",x)



}