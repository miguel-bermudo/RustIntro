struct CustomSmartPointer {
    data: String,
}

// Drop runs when the smart pointer goes out of scope, instead of calling Dealloc, rust runs drop instead.
// this is usefull if the pointer is using shared data or something of the sorts and u want to free it.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn test_pointer() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    
}// they go out of scope here and the code fro Drop would run.

pub fn early_drop_call(){
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // Drop cannot be called directly from the instance.
    // c.drop();
    //however, we do have a standard library call for it
    drop(c);
    println!("CustomSmartPointer dropped before going out of scope.");
}