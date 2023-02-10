pub struct CharStream {
    content: String,
    pointer: usize,
}

impl CharStream {
    pub fn is_empty(&self) -> bool {
        self.pointer >= self.content.len()
    }

    pub fn next(&mut self) -> Option<char> {
        let x = self.peek();
        self.pointer += 1;
        return x;
    }

    pub fn peek(&self) -> Option<char> {
        if let Some(x) = self.content.chars().nth(self.pointer) {
            Some(x)
        } else {
            None
        }
    }

    pub fn new(data: String) -> CharStream {
        CharStream {
            content: data,
            pointer: 0,
        }
    }
}
