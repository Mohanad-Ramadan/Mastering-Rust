fn main() {

    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32

    //MARK: - Mathematical Operation
    // addition
    let _sum = 5 + 10;
    
    // subtraction
    let _difference = 95.5 - 4.3;
    
    // multiplication
    let _product = 4 * 30;
    
    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    
    // remainder
    let _remainder = 43 % 5;
    
    //MARK: - Char
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
    
    //MARK: - Tuple
    let tup = (3, "" , true);

    let (_x, _y, _z) = tup;

    let _tup_string = tup.1;

    let mut mutated_tup: (i32, i32) = (1, 2);
    mutated_tup.0 = 0;
    mutated_tup.1 += 5;


    //MARK: - Array
    let _months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    let _array_one: [i32; 5] = [1, 2, 3, 4, 5];
    let _array_two = [3; 5];
}
