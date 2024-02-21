fn main() {
    // SIMPLE ENUM DECLARTATION, YOU CAN DEFINE TYPE OF THE VARIANT (USSTATE)
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    // MATCH PATTERN AND ARMS
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            // match keyword followed by expression, in here value coin
            // it feels simmilar to if, but big difference is that if statement need to return
            // boolean, match can deal anything, like in here enum type Coin
            Coin::Penny => {
                println!("Lucky penny!");
                1
            } // these are called match arms
            Coin::Nickel => 5, // arms has two parts, pattern and code
            Coin::Dime => 10,  // here pattern is Coin::Dime, and the code is just returned value
            //
            Coin::Quarter(state) => {
                // in here when you input a Coin::Quarter(UsState::Alska)
                // the value of that Quarter 'Alska will bind to the code block of the arm'
                // In this way you extracted the inner state value our of the coin enum.
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    // MATCHIN PATTERN WITH OPTION<T>. HOW TO GET THE INNER VALUE FROM OPTION ENUM?
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, // matching patterns in rust are exhaustive meaning you need to handle
            // every posssible option, so here in match with Option, you need to handle None(null)
            // possiblity otherwise the compiler with shout at you!
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // THE _ PLACEHOLDER - HOW TO DEAL WITH SITUATION WHEN WE DON'T WONT TO CREATE A MATCH ARM FOR
    // EVERY POSSIBLE OPTION? U8 HAVE 0 THROUGH 255 POSSIBLE VALUES. IF WE ONLT CARE ABOUT A FEW:
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // The _ pattern will match any value, by pytting it after other arms, the _ will
                 // match all the possible cases that aren't specified before it. The () is just the unit
                 // value, so nothing will hapen in the _ case.
    }

    // CONCISE CONTROL FLOW WITH IF LET
    // if let lets you combine if and let to handle values that match one pattern and ignore the
    // rest
    let some_u8_value = Some(0u8);
    // that's how you do it with match
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // and with if let, less boilerplate, more concise but loosing exhaustive checking that match
    // enforces, it's kind of syntatic sugar for match
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // IF ELSE
    // with match
    fn if_else_coin(coin: Coin) -> u32 {
        let mut count = 0;
        // match coin {
        //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        //     _ => count += 1,
        // }

        if let Coin::Quarter(state) = coin {
            println!("State Quarter from {:?}!", state);
            count   // Changed from 8 to count, assuming you meant to increment count again
        } else {
            count += 1;
            count   // Return count after incrementing it
        }
    }

    // THE BELOW SNIPPET WON'T COMPILE (ERROR), BECAUSE THE Y CAN BE NULL!
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
    // end of the snippers

    // DEBUG THE STATE
    #[derive(Debug)] // this is to inspect state
    enum UsState {
        Alabama,
        Alaska,
    }
}
