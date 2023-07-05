// The type statement can be used to give a new name to an existing type. Types must have UpperCamelCase names, or the compiler will raise a warning. The exception to this rule are the primitive types: usize, f32, etc.

type Integer = i32;
type Float = i32;

type CombineType = i32;

fn main() {
 
    let nanoseconds: Integer = 5 as CombineType;
    let inches: Float = 2.9 as CombineType;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}

// Output
// 5 nanoseconds + 2.9 inches = 7 unit?