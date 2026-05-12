struct Editor {
    left: Vec<char>,
    right: Vec<char>,
}

impl Editor {
    fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    // insert list of characters at cursor
    fn insert_word(&mut self, word: &str) {
        for c in word.chars() {
            if c != '\0' {
                self.insert_character(c);
            } else {
                break;
            }
        }
    }

    // insert character at cursor
    fn insert_character(&mut self, c: char) {
        self.left.push(c);
    }

    // move cursor left
    fn move_left(&mut self, pos: usize) {
        let mut leftsize = self.left.len();
        while pos != leftsize {
            self.right.push(self.left.pop().unwrap());
            leftsize = self.left.len();
        }
    }

    // move cursor right
    fn move_right(&mut self, pos: usize) {
        let mut rightsize = self.right.len();
        let mut i = 1;
        if pos > rightsize {
            println!("Cannot move the cursor right to the specified position");
        } else {
            while i <= pos {
                self.left.push(self.right.pop().unwrap());
                i += 1;
            }
        }
    }

    fn move_cursor(&mut self, pos: usize) {
        let leftsize = self.left.len();
        let rightsize = self.right.len();

        if pos < leftsize {
            self.move_left(pos);
        } else {
            self.move_right(pos - leftsize);
        }
    }

    fn backspace(&mut self) -> bool {
        if self.left.is_empty() {
            return false;
        } else {
            self.left.pop();
        }
        return true;
    }

    fn delete(&mut self) -> bool {
        if self.right.is_empty() {
            return false;
        } else {
            self.right.pop();
        }
        return true;
    }

    fn get_string(&self) -> String {
        let mut s = String::new();
        for c in &self.left {
            s.push(*c);
        }
        for c in self.right.iter().rev() {
            s.push(*c);
        }
        s
    }

    fn examine_top(&self) {
        let left: String = self.left.iter().collect();
        let right: String = self.right.iter().rev().collect();

        println!("{}|{}", left, right);
    }

    fn find_and_replace(&mut self, find_what: char, replace_with: char) {
        let mut count = 1;
        let cursorpos = self.left.len();
        self.move_cursor(0);

        while self.right.is_empty() != true {
            if self.right[self.right.len() - 1] == find_what {
                self.delete();
                self.insert_character(replace_with);
            } else {
                self.move_cursor(count);
            }
            count += 1;
            self.move_right(1);
        }
        self.move_cursor(cursorpos);
    }
}
