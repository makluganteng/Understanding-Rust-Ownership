fn main() {
    println!("Hello, world!");
    func1();
    func2();
    func3();
    func4();
    test();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
    test8();
}

fn func1() {
    let s1 = String::from("hello");
    {
        //This can be declared twice because it is not used outside of the scope
        let mut s1 = String::from("hello");
        s1.push_str(", world!");
    } //everything inisde the scope is destroyed here
    println!("{}", s1);
}

//for integers , boolean , float , char , tuples all can be copied and not moved
fn func2() -> i32{
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    y
}

//this will be error because s1 is moved to s2
// fn func3() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}, world!", s1);
// }

//best way to fix this is to clone it 
//so what happen here is that If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone
fn func3() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

//the other way to do this is using reference
fn func4() {
    let s1 = String::from("hello");
    let len = &s1.len();
    println!("The length of '{}' is {}.", s1, len);
}

//Ownerships and Functions
//so here when a value is moved and use as argument of a function like this is still possible because the value will be dropped after the function is done
//so memeor will be freed
fn test() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


//here in the function gives_ownerhip it gives the value being declared and return it.
//but then in s3 it takes_and_gives_back function will take the value and return it.
//so here it takes in the value which i can say is moved but then its returned again after the end of the function.
fn test2() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

//in a fuction u can return 2 values like the example below
fn test3() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length1(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length1(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

//refernece and borrowing 
//so here we are using & to make a reference to the value of s1
// so first what happen is that the value beig used in calsulate_length is not moved
// because its using refenrence from the s1 value
// so the value of s1 is not moved and the value of s1 is still valid after the function is done

//The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. 
//Because it does not own it, the value it points to will not be dropped when the reference stops being used.
fn test4() {
    let s1 = String::from("hello");

    let len: usize = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

//Mutable References
//here is an example but it will not work because we are trying to change the value of s1
//here the value of s1 is immutable so we cannot change it
// fn test5() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

//here is the solution if we want to change the value of s1
//so here we are using &mut s1 to make it mutable
fn test5() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//there is restriction on the number of mutable references to a particular piece of data in a particular scope
//for example 
// let mut s = String::from("hello");

// let r1 = &mut s;
// let r2 = &mut s;

// println!("{}, {}", r1, r2);
//This will not be able to work because the value of s is mutable and we are trying to use it twice

//so here is the solution
//so what it did here is that inside the curly braces is a new scope
//so when r1 goes out of scope it will be dropped and then we can use r2
fn test6() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

//here is also 1 more of bad example of using mutable reference
// let mut s = String::from("hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem
// let r3 = &mut s; // BIG PROBLEM

// println!("{}, {}, and {}", r1, r2, r3);
//this will not work because we are trying to use the value of s as mutable and immutable at the same time
//because s is referenced as immutable and mutable at the same time

//so here is the solution
//so in this case the value of r1 and r2 is no longer being used to now r3 can have &mut s without any problem.
fn test7() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

}

//Dangling References
// so here is an example of dangling references
//so here the value of s is being returned but then the value of s is dropped
//so the value of s is dropped but then the value of s is still being used
//so this will not work
// fn test8() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

//here is also 1 other mistake people usually does
// fn test8() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

//so the best solution for this is to just return string immediately
fn test8() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    s 
}

//so there are times we need to use refernece or clone. But there are times to better just return the value immediately without reference or clone.

//Slicing 
//This function is to retunr the length of the first word in a string
fn first_word(s: &String) -> usize {
    //Because we need to go through the String element by element and check whether a value is a space, we’ll convert our String to an array of bytes using the as_bytes method.
    let bytes = s.as_bytes();

    //Next, we create an iterator over the array of bytes using the iter method:
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

//so in this function u can see that we are using &String as a parameter but since word and s is not correlated in the first place so we can clear the value of s
fn test9() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}





