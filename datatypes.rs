fn main(){
    // datatypes in rust:-
    //1. scaler type:-
        //a. - integers
            let num = 34; 
            // range signed interger :- -2^n-1 to 2^n-1 - 1
            // range unsigned interger :- 0 to 2^n-1
            
        //b. - booleans
            // 1byte size
            let t = true;
            let f:bool = false; // with expicit type annotation

        //c. - floating point numbers
            // the default type is f64
            let decnum = 3.4; // f64 double precision float
            let decnum2 :f32 = 3.0; // f32 single precision float

        //d. - characters
            // stored in single qoutes
            // 4 bytes size
            let c = 'z';
            let z: char = 'ℤ'; // with explicit type annotation
            let heart_eyed_cat = '😻';
        
        

    //2. compound type:-
        //a. - tuple
            let tup:(i32,f64,u8) = (500,6.4,1);

            // pattern matching to destructure a tuple value
            let tup_2 = (500,3.4,1);
            let (x,y,z) = tup_2;
            println!("The value of y is: {y}");

            // access elements of a tuple by using (.) period
            let x: (i32, f64, u8) = (500, 6.4, 1);
    
            let five_hundred = x.0;
            println!("{}",five_hundred);
        
            let six_point_four = x.1;
            println!("{}",six_point_four);
        
            let one = x.2;
            println!("{}",one);


        //b. - array
            // every element of an array must have same type
            // fixed length
            let a = [1,2,3,4,5];

            // synatx 
            let a:[i32;5] = [1,2,3,4,5];

            // initializing an array to contain same value each element by specifying the initial value
            let a = [3;5]; // same as let a = [3,3,3,3,3]

            // accessing array elements
            let a = [1,2,3,4,5];
            let first = a[0];
            let second = a[1];

    // numeric operations
    fn main() {
        // addition
        let sum = 5 + 10;
    
        // subtraction
        let difference = 95.5 - 4.3;
    
        // multiplication
        let product = 4 * 30;
    
        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1
    
        // remainder
        let remainder = 43 % 5;
    }
    





    


}