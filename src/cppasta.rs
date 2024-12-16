use std::{io::Write, thread::sleep, time::Duration};

use copypasta::x11_clipboard::{Primary, X11ClipboardContext};
use copypasta::{ClipboardContext, ClipboardProvider};

pub fn copy_pasta_func() {
    let mut ctx = ClipboardContext::new().unwrap();
    println!("\n\n");
    let mut content = "Empty".to_string();

    loop {
        let new_content = ctx.get_contents().unwrap();
        if content != new_content {
            content = new_content;
        }
        print!("Selected text: {}                   \r", content);
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(100));
    }
}

pub fn get_selection() {
    let mut ctx = X11ClipboardContext::<Primary>::new().unwrap();

    println!("\n\n");
    let mut content = "Empty".to_string();
    ctx.set_contents(content.clone()).unwrap();
    loop {
        let new_content = ctx.get_contents().unwrap();
        if content != new_content {
            content = new_content;
            println!("Selected text: {}\n Len:{}", content, content.len());
            //std::io::stdout().flush().unwrap();
        }
        sleep(Duration::from_millis(100));
    }
}
