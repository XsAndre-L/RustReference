

// Primitive Data Types in Rust


#[test] // <--- This makes it a test function
#[allow(unused_variables)] // <--- This silences "you didn't use this variable" warnings
#[allow(unused_mut)]       // <--- Silences "you didn't need to make this mutable" warnings
pub fn data_type_reference() {
// 1. SCALAR TYPES -------------------------------
// A single value stored on the stack.

// Integers (Signed)
const signed_8bit: i8 = 8;
const signed_16bit: i16 = 16;
const signed_32bit: i32 = 32;
const signed_64bit: i64 = 64;
const signed_128bit: i128 = 128;

// Integers (Unsigned)
const unsigned_8bit: u8 = 8;
const unsigned_16bit: u16 = 16;
const unsigned_32bit: u32 = 32;
const unsigned_64bit: u64 = 64;
const unsigned_128bit: u128 = 128;

const size_signed: isize = 16;
const size_unsigned: usize = 16;

// Floats
const float_32bit: f32 = 32.0;
const float_64bit: f64 = 64.0;

const boolean: bool = true;

const character: char = 'c';


// 2. COMPOUND TYPES ------------------------------
// Fixed-size groups of values.

// Arrays (Fixed length, same type)
// Signature: [Type; Length]
const array_slice: &[i32] = &[1, 2, 3];

// Tuples (Fixed length, can be different types)
const tuple: (i32, f64, char) = (42, 3.14, 'a');

// Unit Type (An empty tuple, returns nothing)
const unit_type: () = ();


// 3. SEQUENCE TYPES (Slices) ---------------------
// References to a sequence of elements in memory.

// String Slice (Reference to UTF-8 data)
const string_slice: &str = "string slice";
// Note: 'str' alone is a Dynamically Sized Type (DST) and cannot be a variable.


// 4. POINTER TYPES -------------------------------

// References (Safe pointers)
let reference: &i32 = &10;
let mut x = 10;
let mutable_reference: &mut i32 = &mut 10;

// Raw Pointers (Unsafe C-like pointers)
// Essential for FFI (Foreign Function Interface) or low-level optimizations
let raw_const: *const i32 = &10 as *const i32;
let raw_mut: *mut i32 = &mut 10 as *mut i32;


// Function Pointers
// Points to code, not data.
fn add_one(x: i32) -> i32 { x + 1 }
let fn_ptr: fn(i32) -> i32 = add_one;

fn example_function<T, U>(x: T) -> U {
    unimplemented!()
}

const function_item: fn(i8) -> i8 = example_function::<i8, i8>;
const function_pointer: fn(i8) -> (i8) = example_function::<i8, i8>;

const closure: fn(i32) -> i32 = |x: i32| -> i32 { x + 1 };


}


// 5. SPECIAL TYPES ---------------------------------

// The Never Type (!)
// Represents a computation that never completes (e.g., panic, infinite loop, exit).
// You cannot create a variable of this type.
fn crash() -> ! {
    panic!("This function never returns");
}