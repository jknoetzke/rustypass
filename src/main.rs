use arboard::Clipboard;

fn main() {
    copy_to_clipboard("Foobar!".to_string())
}

fn copy_to_clipboard(str: String) {
    let mut clipboard = Clipboard::new().unwrap();
    println!("Clipboard text was: {}", clipboard.get_text().unwrap());

    clipboard.set_text(&str).unwrap();
    println!("But now the clipboard text should be: \"{}\"", str);
}
