use window::KeymojiInputEvent;

mod text;
mod window;

#[derive(Debug)]
enum InputModel {
    Init,
    QueryEntered { query: String },
    Results { query: String, results: Vec<String> },
}

impl window::ViewStateModel for InputModel {
    fn update(&self, input: KeymojiInputEvent) -> Self {
        match input {
            KeymojiInputEvent::KeyPressed(code) => match self {
                Self::Init => Self::QueryEntered {
                    query: format!("{:?}", code),
                },
                InputModel::QueryEntered { query } => InputModel::QueryEntered {
                    query: format!("{} {:?}", query, code),
                },
                _ => InputModel::Init,
            },
            _ => InputModel::Init,
        }
    }
}

fn main() {
    text::example();
}
