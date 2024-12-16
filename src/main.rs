pub mod cppasta;
pub mod ollama_mod;
pub mod iced_window;

use copypasta::x11_clipboard::{Primary, X11ClipboardContext};
use copypasta::ClipboardProvider;


#[tokio::main]
async fn main(){

    println!("Assuming model llama3.2 is already installed from ollama");
    let mut ctx = X11ClipboardContext::<Primary>::new().unwrap();

    println!("\n\n");
    let mut content = "".to_string();
    ctx.set_contents(content).unwrap();
    let mut printed = false;
    iced_window::run_iced_window().await.unwrap();
    loop {
        
    }
}