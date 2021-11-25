pub trait Draw {
    fn draw(&self) {}
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

pub struct Button {
    pub button_text: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw the button, button_text = {}", self.button_text);
    }
}

pub struct TextBox {
    pub title: String,
}

impl Draw for TextBox {
    fn draw(&self) {
        println!("Draw the TextBox, title = {}", self.title);
    }
}
