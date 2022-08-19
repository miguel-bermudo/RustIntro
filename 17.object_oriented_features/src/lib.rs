// using trait objects to implement a GUI.
// draw is any object such as a button, select etc that has to be drawn to screen.
pub trait Draw {
    fn draw(&self);
}

// it is the screen layout etc, the entry point of the GUI
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

//the button component definition
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}