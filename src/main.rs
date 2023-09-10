fn main() {
    let mut s1: String = String::from("Hello");
    // change(&s1); // this line throws an error. The function change() attempts to change a borrowed value. Only values that are owned can be changed.
    allowed_change(&mut s1);
    // two_mutable_references_will_fail();
    two_mutable_references_in_diff_scopes();
    // borrow_after_immutable_reference();
    references_outofscope_lasttime_value_used();

    let len: usize = calculate_len(&s1);
    println!("The length of String s1 = {}", len);
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

// fn change(s: &String) {
//     /**
//      * This will throw an error!
//      * This value is referenced, and thus borrowed. 
//      * We do not own a borrowed value, and thus this code will throw an error.
//      * We can only modify values that are owned.
//      * Also, even if this function is not used, the program still does not compile.
//      * The compiler sees that s is borrwed and attempted to be modified.
//      * Not only is this a borrowed value, but variables are immutable by default.
//      * In this example, we have declared s1 in main() as immutable, without the mut keyword.
//      */
//     s.push_str(", world!");
// }

fn allowed_change(s: &mut String) {
    /**
     * This change is allowed.
     * The function definition specifies the value is not borrowed,
     * but we actually obtain ownership, allowing us to modify the value.
     * The String variable passed as argument s must be specified as let mut in the main() function,
     * because variables are immutable by default.
     */
    s.push_str(", world!");
}

// fn two_mutable_references_will_fail() {
//     let mut s1: String = String::from("Hello, world!");
//     let r1: &mut String = &mut s1;
//     let r2: &mut String = &mut s1;

//     println!("This function will cause a compile time error.");
//     println!("Because two mutable references to the same value are not allowed!");
//     println!("Interesting, if the double mutable references to s1 are not used, no compile time error is thrown. Need to use the print statement below.");
//     println!("r1 = {}, r2 = {}", r1, r2);
//     println!("There we go, as expected the compiler is not happy.");
//     println!("Even if this function is not used, the compiler will still throw an error.");
// }

fn two_mutable_references_in_diff_scopes() {
    let mut s1: String = String::from("Hello, world!");
    
    {
        let mut r1: &mut String = &mut s1;
        println!("mut r1 is used! = {}", r1);
    }

    let mut r2: &mut String = &mut s1;

    // ^ This is fine because although we have two mutable references,
    // r1 goes out of scope.
    // Remember that when r1 goes out of scope, because it is a mutable borrowed reference, s1 is not dropped.
    // Thus r2 can become the mutable reference to s1.

    println!("mut r2 is used! = {}", r2);
}

fn borrow_after_immutable_reference() {
    let mut s1: String = String::from("Hello, world!");
    let r1: &String = &s1;   // this is OK. We cannot change the data.
    let r2: &String = &s1;   // this is OK. We cannot change the data.
    // both of these immutable borrows/references are OK because the value of s1 does not change later, nor does a mutable borrow of this value occur.
    // let mut r3: &mut String = &mut s1;  // as long as we don't actually change this value, no error is thrown, because the immutable reference of r1 and r2 will still read the same value. It is never updated.
    // s1.push_str("Error thrown?");
    // the s1 push also does not work, because the compiler recognizes that there has been an immutable reference(s) of this value previously, and s1 is not allowed to change, whether it changes itself or a mutable borrow occurs and that value is later changed.

    // println!("r1, r2, r3 = {}, {}, {}", r1, r2, r1);
    // println!("r1, r2, r3 = {}, {}, {}", r1, r2, s1);
    // OK, this now throws an error. Even though r3 is not actually changed, it is used after the immutable references, and the compiler throws an error. This is so the immutable references that r1 and r2 point to can possibly be changed in code later, and the compiler doesn't care if it's even a println!
}

fn references_outofscope_lasttime_value_used() {
    /**
     * Ok, this one is a bit tricky.
     * r1 and r2 here are immutable references.
     * However, they are used in a println! before a mutable borrow occurs later.
     * Because they are used, these variables go out of scope that last time they are used within the given scope.
     * So creating a mutable borrow after r1 and r2 are last used is OK.
     */

    let mut s1: String = String::from("Hello world!");
    let r1: &String = &s1;
    let r2: &String = &s1;

    println!("r1, r2 = {}, {}", r1, r2);
    println!("Now r1 and r2 have been used for the last time in this function, and they are thus out of scope. The compiler recognizes this.");

    let mut r3: &mut String = &mut s1;
    s1.push_str(", we modified s1!");
    println!("We modified s1 after the immutable references were last used in this function (and the compiler recognizes that they are out of scope at that point because they are no longer used.");
}