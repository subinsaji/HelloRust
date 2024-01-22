fn main() {
    let x = 5;
    let x = x +1;

    {
        let x = x * 2;
        println!("The value of the x in the inner scope is: {x}");

    }

    println!("The value of x is {x} in outer scope");

    let spaces = "   ";
    //let spaces = spaces.len();

    // tuples

    let tup = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("The value of x,y and z is {x}, {y} and {z} respectively")

}

fn another_function(){
    println!("Another function");
    
}
