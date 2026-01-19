/*  - Convention is to use snake case for functions
 *  - Can define functions either before or after callers, just needs to be in same scope
 *  - MUST define arg type in function
 */

fn main() {

   test_function(5);
   let return_val = return_function(5);
   println!("{return_val}");
}

fn test_function(x:i32){
    println!("{x}");

//  let y = (let x = 6) is invalid unlike C
//  Need a macro to eval that first, similar to a function call
//  No ; on final line to make the macro return a value
    let y = {
        let x =x;
        x+1};

    println!("{x},{y}");
}

// Need to define type for return
// Funcs will implicitly return last value BUT can be returned early using return
fn return_function(x:i32) -> i32 {
    x   // Note no ; as that change line from expression to statement
}


