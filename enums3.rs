#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

#[derive(Debug)] // ✅ Adding Debug here
struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::Resize { width, height } => self.resize(width, height),
            Message::Move(point) => self.move_position(point),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(r, g, b) => self.change_color(r, g, b),
            Message::Quit => self.quit(),
        }
    }
}

fn main() {
    let mut state = State {
        width: 0,
        height: 0,
        position: Point { x: 0, y: 0 },
        message: String::from("hello world"),
        color: (0, 0, 0),
        quit: false,
    };

    state.process(Message::Resize {
        width: 800,
        height: 600,
    });
    state.process(Message::Move(Point { x: 100, y: 200 }));
    state.process(Message::Echo(String::from("Rust is great!")));
    state.process(Message::ChangeColor(128, 64, 255));
    state.process(Message::Quit);

    // ✅ This now works
    println!("State after processing messages: {:?}", state);
}
