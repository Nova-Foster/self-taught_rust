fn main() {
   scalar_types();
   compound_types();
}

fn scalar_types(){
    // Represents single value
    
    // Integers
    /* Format: i / u - bit size
     *  - i for signed & u for unsigned
     *  - bit size of 8, 16, 32, 64, 128
     *  - Default of i32
     */

    // Floats of form f32 or f64
    
    // Booleans are bool, usually won't need to be explicitly declared

    // Char specified using ' ' & is 4 bytes
}

fn compound_types(){
    // Multiple values

    // Tuple
    /* Format: (data_type, data_type,...) = (val, val,...)
     * - Fixed size after being declared
     * - Multiple types are allowed
     * - Access value tuple.position
     * - Empty Tuple has a special 'Unit' type
     */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup_0 = tup.0;
    println!("{tup:#?}");
    println!("Index 0 = {tup_0}");      // Note that you can't use tup.0 here, has to be assigned
                                        // elsewhere first
   
    // Array
    /* Format: name = [vals,vals....]    or name: [type; #elements] = [vals] or name: a:[val;#] for
     *                                                                                      same val
     *  - Fixed length at declaration
     *  - All the same data type
     *  - Index as name[index]
     * */
}
    
