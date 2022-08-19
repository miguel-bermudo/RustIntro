use object_oriented_features::{Draw, Screen, Button};

// someone continuin to implement the GUI library 
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

pub fn run_gui(){
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

pub fn screen_with_non_draw(){
    // as string does not implement the draw trait we cannot embed it into the list of components.
    
    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };

    // screen.run();
}