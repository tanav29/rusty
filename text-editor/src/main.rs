use crate::editor::Editor;
use std::io::{self, Read};

fn main() {
    let mut editor = Editor::new();

    editor.insert('h');
    editor.insert('e');
    editor.insert('l');
    editor.insert('l');
    editor.insert('o');

    editor.debug_view();

    editor.move_left();
    editor.move_left();

    editor.debug_view();

    editor.insert('X');

    editor.debug_view();

    println!("Final text: {}", editor.get_text());
}
