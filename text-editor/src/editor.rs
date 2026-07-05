#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Find,
}

pub struct Editor {
    left: Vec<char>,
    right: Vec<char>,
    pub command: String,
    pub mode: Mode,
    pub posx: isize,
    pub posy: isize,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
            mode: Mode::Normal,
            command: String::new(),
            posx: 0,
            posy: 0,
        }
    }

    // insert list of characters at cursor
    pub fn insert_word(&mut self, word: &str) {
        for c in word.chars() {
            if c != '\0' {
                self.insert_character(c);
            } else {
                break;
            }
        }
    }

    // insert character at cursor
    pub fn insert_character(&mut self, c: char) {
        self.left.push(c);
        self.posx += 1;
    }

    // move cursor left
    pub fn move_x(&mut self, delta: isize) {
        if delta > 0 {
            self.move_right(delta as usize);
        } else if delta < 0 {
            self.move_left((-delta) as usize);
        }
    }

    // move cursor left by `count` characters
    pub fn move_left(&mut self, count: usize) {
        for _ in 0..count {
            match self.left.pop() {
                Some(c) => {
                    self.right.push(c);
                    self.posx -= 1;
                }
                None => break,
            }
        }
    }

    // move cursor right by `count` characters
    pub fn move_right(&mut self, count: usize) {
        for _ in 0..count {
            match self.right.pop() {
                Some(c) => {
                    self.left.push(c);
                    self.posx += 1;
                }
                None => break,
            }
        }
    }

    pub fn move_down(&mut self) {
        let mut column = 0;

        // Find current column.
        while let Some(&c) = self.left.last() {
            if c == '\n' {
                break;
            }
            self.move_left(1);
            column += 1;
        }

        // Restore original position.
        self.move_right(column);

        // Move to start of next line.
        let mut found_newline = false;
        while let Some(&c) = self.right.last() {
            self.move_right(1);
            if c == '\n' {
                found_newline = true;
                break;
            }
        }

        if !found_newline {
            // No next line, stay at current position.
            return;
        }

        self.posy += 1;

        // Restore column on new line.
        let mut moved = 0;
        while moved < column {
            match self.right.last() {
                Some('\n') | None => break,
                _ => {
                    self.move_right(1);
                    moved += 1;
                }
            }
        }
    }

    pub fn move_up(&mut self) {
        // Distance from cursor to start of current line.
        let mut column = 0;

        while let Some(&c) = self.left.last() {
            if c == '\n' {
                break;
            }
            self.move_left(1);
            column += 1;
        }

        // Already on first line.
        if self.left.is_empty() {
            return;
        }

        // Cross newline to previous line.
        self.move_left(1);

        // Go to start of previous line.
        while let Some(&c) = self.left.last() {
            if c == '\n' {
                break;
            }
            self.move_left(1);
        }

        self.posy -= 1;

        // Restore column without crossing newline.
        let mut moved = 0;
        while moved < column {
            match self.right.last() {
                Some('\n') | None => break,
                _ => {
                    self.move_right(1);
                    moved += 1;
                }
            }
        }
    }

    pub fn backspace(&mut self) -> bool {
        if self.left.is_empty() {
            return false;
        } else {
            self.left.pop();
            self.posx -= 1;
        }
        return true;
    }

    pub fn delete(&mut self) -> bool {
        if self.right.is_empty() {
            return false;
        } else {
            self.right.pop();
        }
        return true;
    }

    pub fn get_string(&self) -> String {
        let mut s = String::new();
        for c in &self.left {
            s.push(*c);
        }
        for c in self.right.iter().rev() {
            s.push(*c);
        }
        s
    }

    pub fn examine_string(&self) -> String {
        let left: String = self.left.iter().cloned().collect();
        let right: String = self.right.iter().rev().cloned().collect();
        format!("{}|{}", left, right)
    }

    pub fn cursor_position(&self) -> usize {
        self.left.len()
    }
}
