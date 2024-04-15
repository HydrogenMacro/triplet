/// # Triplet
/// Triplet is an esoteric language for code golfing designed to be as compact as possible.
/// Its instructions are organized into categories and subcategories (think of it like an org chart),
/// and they all take 3 bits to access.
/// ## Language Features
/// Triplet's input and output are all on a stack of unsigned 8-bit integers.
/// The stack will initially have the input ASCII, and the output will be what remains on the stack
/// Triplet has 3 main data types: numbers, an array, and functions
/// ## Main Categories
/// ### *1s and 0s represent bits in the calling script*
/// 000 - Numbers
/// 001 - Array
/// 010 - Operations
/// 011 - Stack Procedures
/// 100 -
/// 101 -
/// 110 - Special Procedures
/// 111 - Terminate
/// ## Example Programs
/// ### Hello World!
/// ```
/// 000
/// 010
///
/// ```

fn main() {
    let triplet_code = vec![0b111_111u8];
    let triplet_output = triplet_parse(&triplet_code);
    println!("{:?}", triplet_output)
}
fn triplet_parse(data: &[u8]) {

}