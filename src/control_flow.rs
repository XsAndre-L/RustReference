

pub fn control_flow_reference() {
    // Control Flow Constructs in Rust

    // 1. CONDITIONALS -------------------------------

    let number = 6;

    // if-else statement
    if number == 0 {
        println!("number is zero");
    } else if number == 3 {
        println!("number is 3");
    } else if number == 2 {
        println!("number is 2");
    } else {
        println!("number is something else");
    }

    // 2. LOOPS --------------------------------------

    // infinite loop 
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break; // infinite unless we break
        }
    }

    // while loop
    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // ---------------------------------------------------------
    // 3. MATCH (The C++ Switch on Steroids)
    // ---------------------------------------------------------
    // Match must be EXHAUSTIVE (must cover every possible value).
    
    let input = 42;

    match input {
        1 => println!("One"),
        2 | 3 | 5 | 7 => println!("Prime"), // Multiple matches
        13..=19 => println!("A teen"),      // Ranges
        x if x % 2 == 0 => println!("Even"), // Guard Clause (Custom logic)
        _ => println!("Catch all"),         // Default case (required)
    }

    // Match is also an expression!
    let boolean_val = true;
    let binary = match boolean_val {
        true => 1,
        false => 0,
    };

}