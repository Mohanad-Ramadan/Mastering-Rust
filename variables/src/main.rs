const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 3; this will cause an compiler error
    // println!("The value of x is: {x}");

    let mut y = 2;
    println!("The value of y is: {y}");
    y = 4;
    println!("The value of y is: {y} \n");

    // constants
    println!("{THREE_HOURS_IN_SECONDS} \n");

    // shadowing
    let x_shadow = "hello shadow";
    print!("before shadowing: {x_shadow} \n");
    let x_shadow = x_shadow.chars().as_str();

    {
        // here shadowing allow to change the type, and mutation doesn't allow that 
        let x_shadow = x_shadow.len();
        println!("inner scoop shadwing: {x_shadow} \n");
        // after this inner scoop is over, x_shadow will return to it's previous type and value
    }

    // strong points of shadowing 
        // scoop lifecycle
        // utilize single variable name
        // typecasting
    let x_shadow = x_shadow.ends_with("shadow");
    println!("after inner scoop over: {x_shadow} \n");


    // we can change it to a mutable variable or vise versa
    let mut_shadowed_x = 2;
    let mut mut_shadowed_x = mut_shadowed_x + 2;
    mut_shadowed_x += 2;
    println!("it's mutable now: {mut_shadowed_x}")
}
