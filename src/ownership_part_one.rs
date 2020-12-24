/*
The stack and the heap are 2 parts of memory. Memory is available for your code at runtime (when you program runs)
Depending on what your running program needs, it will either address the stack or the heap.

The stack is very efficient. The stack stores values in order and removes them in reverse order.
This is very fast because the OS never has to search for the value: it just pops it off the stack.

Data size on the stack must be fixed and known at compile time, like primitives and pointers.

Data that isn't fixed, will get stored on the heap. This is data that can change overtime, like strings, vectors, objects.
Data on the heap is less organised. Your OS will find a empty spot in memory that is big enough, and stores the data.
Your system will then return a pointer. A pointer is a piece of data that points to the data on the heap. 
Because the pointer has a fixed size, it gets stored on the stack. 

Getting data from the heap is a much slower process, because your system has to search for the data in memory.

Rust has a special feature, called ownership. There are 3 main rules for ownership:
1. Each value in Rust has a variable called its owner
2. There can be only one owner at a time
3. When the owner goes out of scope, the value will be dropped.


*/

pub fn _scope() {
    // x "owns" 1
    let _x = 1;

    {
        // variables are scoped, and will drop after it goes out of scope
        let _a = 10;
    }

    // cannot find a because the value is out of scope
    // x + a;
}

pub fn _copy() {
    // You can allocate a value of a variable to another variable like so:
    let a = 10;
    let _b = a;

    println!("{}", a);
    println!("{}", a);

    // both a and b have the value of 10, but both values are allocated to a different spot in memory
    // this is called copying using the copy trait. You can only copy data stored on the stack.

    // example 2
    let c; // we create variable c without a value.

    { // we create a new scope
        let d = 20; // d gets defined within this scope
        c = d; // the value of d gets copied to variable c
    } // d goes out of scope and the value will be dropped

    println!("c is : {}", c); // c still holds the value of 20 thanks to copy
}

pub fn _strings() {
    // this is type String
    let mut s = String::from("String");

    // these are string literals
    let sl = "I have a fixed size";

    // difference between String and &str is that String can grow over time, and gets stored on the heap
    // &str is fixed and is immutable, and gets stored on the heap. You can "push" &str on String:
    
    s.push_str(sl);
    println!("{}", sl);

    /*
    Data on the heap will return a pointer. 
    Pointers are stored on the stack, so you might think that it's possible to copy pointers as well. 
    But duplicate can cause data corruption when they both try to free memory when they go out of scope.
    Which is why types like strings will 'move' to another variable, and will cause an error when you try to call new_s:
    */
    let new_s = String::from("Hello");
    let _t = new_s;

    // println!("{}", new_s);
}

pub fn _pointers() {
    /*
    here you can see that both a and b hold value of 10, but in a different spot in memory:
    */
    let a = 10;
    let b = a;

    println!("{:p}", &a);
    println!("{:p}", &b);

}

