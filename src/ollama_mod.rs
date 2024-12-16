use ollama_rs::{
    generation::completion::request::GenerationRequest,
    Ollama,
};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
pub async fn run_llama3_2(prompt: String){
    // By default it will connect to localhost:11434
    let ollama = Ollama::default();

    //let gen_options = GenerationOptions::default();
    
    // For custom values:
    //let ollama = Ollama::new("http://localhost".to_string(), 11434);

    let model = "llama3.2".to_string();
    
    let mut stream = ollama.generate_stream(GenerationRequest::new(model, prompt)).await.unwrap();

    let mut stdout = tokio::io::stdout();
    while let Some(res) = stream.next().await {
        let responses = res.unwrap();
        for resp in responses {
            stdout.write_all(resp.response.as_bytes()).await.unwrap();
            stdout.flush().await.unwrap();
        }
    }
}