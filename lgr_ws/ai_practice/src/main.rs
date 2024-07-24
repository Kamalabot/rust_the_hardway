use genai::chat::{ChatMessage, ChatRequest};
use genai::utils::print_chat_stream;
use genai::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    // println!("{:?}", client);
    let chat_req = ChatRequest::new(vec![
        ChatMessage::system("You are helpful messager."),
        ChatMessage::user("What is the distance to nearest galaxy."),
    ]);

    let model = "gpt-3.5-turbo";
    println!("\n -- Answer, from model: {model}");
    // the await is used, as the call to api are asynchronous 
    let chat_res = client.exec_chat_stream(model, chat_req, None).await?;
    print_chat_stream(chat_res, None).await?;

    // second version with list of questions
    let mut chat_req1 = ChatRequest::default().with_system("answer with concise hint"); 

    let questions = [
        "when did earth start rotating?",
        "Why did earth start rotating?"
    ];
// for quest in questions{
    println!("\n-- Question: {questions[0]}");

    chat_req1.append_message(ChatMessage::user(questions[0]));

    let chat_res1 = client.exec_chat_stream(model, 
                                                        chat_req1,
                                                        None).await?;

    let chat_res_str = print_chat_stream(chat_res1, None).await?;

    chat_req = chat_req1.append_message(ChatMessage::assistant(chat_res_str));

    Ok(())
}
