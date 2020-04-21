use iced::{Element, Sandbox, Text};

#[derive(Default)]
struct HelloWorld {
    text: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Placeholder,
}

impl Sandbox for HelloWorld {
    type Message = Message;

    fn new() -> HelloWorld {
        HelloWorld {
            text: "Hello, world!",
        }
    }

    fn title(&self) -> String {
        "Sandbox Title".to_string()
    }

    fn view(&mut self) -> Element<Message> {
        Text::new(self.text).size(50).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            _ => {}
        }
    }
}

fn main() {
    HelloWorld::run(Default::default());
}
