// combine imports from the same library into curly brackets.
// glob operator "*" used to import everything from that path. (better to not use it cuz u don't know what u bring into scope.)
use std::{fmt::Result, cmp::Ordering, collections::*}; 
use std::io::{Result as IoResult, self}; // using self imports the library itself.

//since both have Result as the name we can use as to determine an alias for the type.

fn main(){
    fn function1() -> Result {
        // --snip--
        Ok(())
    }
    
    fn function2() -> IoResult<()> {
        // --snip--
        Ok(())
    }
}
