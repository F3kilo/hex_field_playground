use crate::app::Render;

pub struct TermRender {
    buf: String,
}

impl TermRender {
    pub fn new() -> Self {
        Self { buf: String::new() }
    }

    pub fn add_line(&mut self, line: &str) {
        self.buf = format!("\n{} ->> {}", self.buf, line);
    }
}

impl Render for TermRender {
    fn render(&mut self) {
        println!("{}", self.buf);
        self.buf.clear();
    }
}
