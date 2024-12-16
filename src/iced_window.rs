use iced::{
    alignment::{Horizontal, Vertical},
    font::Family,
    widget::{column, text, text::Shaping, Text},
    Font, Length, Sandbox, Settings,
};

pub async fn run_iced_window() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp;

impl Sandbox for MyApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, _message: Self::Message) {
        let new_content = ctx.get_contents().unwrap();
        if  !new_content.is_empty() {
            content = new_content;
            if !printed{
                println!("Selected text: {}\n Len:{}", content, content.len());
                printed = true
            }
            ollama_mod::run_llama3_2(content).await;
            
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        column![
            "Construct from &str",
            text("Construct from function"),
            Text::new("Construct from struct"),
            text("Different font").font(Font {
                family: Family::Fantasy,
                ..Font::DEFAULT
            }),
            text("Larger text").size(24),
            text("Special character 😊").shaping(Shaping::Advanced),
            text("Center")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
            text("Vertical center")
                .height(Length::Fill)
                .vertical_alignment(Vertical::Center),
        ]
        .into()
    }
}