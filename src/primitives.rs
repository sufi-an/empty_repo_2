

pub fn premitives(){
    /* Scalar Types */

    // Integres
    let int_num:i32 = 12;

    // Unsigned Integers
    let unsigned_int_num:u32 = 1234;

    // Floating point
    let float_num:f32 = 3.14159;

    // Char
    let some_char:char = 'a';

    // Bool
    let some_bool:bool = true;

    // The Unit Type
    // let some_empty_tuple:() = ();

    println!("Scalar Types in Rust ->  \n int: {} \n unsigned int: {} \n float: {} \n char: {} \n bool: {} \n empty tuple:  \n",int_num,unsigned_int_num,float_num,some_char,some_bool);

    /* Compound Types */
    // array
    // [type:size] = []/[default]
    let  array: [i32; 5] = [0; 5];

    for x in &array{ 
    println!("{}", x);
    }



}