use std::env;

use muscle_booster::CreatePlanDto;
use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{self, ChatCompletionRequest},
    common::{GPT3_5_TURBO_16K_0613, GPT4_O},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let params = CreatePlanDto {
        weight: 86.0,
        height: 1.70,
        weight_goal: 75.0,
        hours_per_day: muscle_booster::HoursPerDay::Two,
        muscle_goal: muscle_booster::MuscleGoal::Gain,
        training_type: muscle_booster::TrainingType::Gym,
        motivations: vec![
            muscle_booster::Motivation::Healthy,
            muscle_booster::Motivation::Aesthetic,
        ],
    };

    let client = OpenAIClient::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let req = ChatCompletionRequest::new(GPT4_O.to_string(), vec![
        chat_completion::ChatCompletionMessage{
            role: chat_completion::MessageRole::system,
            content: chat_completion::Content::Text("Você irá responder em português e deverá criar um plano de treino baseado nos dados enviados pelo usuário".to_string()),
            name: None,
            tool_call_id: None,
            tool_calls: None,
        },
        chat_completion::ChatCompletionMessage{
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(serde_json::to_string(&params).unwrap()),
            name: None,
            tool_call_id: None,
            tool_calls: None,
        },
    ]);

    let result = client.chat_completion(req).await.unwrap();
    println!("Result: {:#?}", result);

    Ok(())
}
