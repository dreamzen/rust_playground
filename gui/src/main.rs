use gui::{Button, Screen, TextBox};

fn main() {
    let s = Screen {
        components: vec![
            Box::new(Button {
                button_text: "click me".to_string(),
            }),
            Box::new(TextBox {
                title: "read me".to_string(),
            }),
        ],
    };

    for d in s.components {
        d.draw();
    }
}
