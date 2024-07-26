use std::env;

#[macro_use]
extern crate rocket;

use muscle_booster::CreatePlanDto;
use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{self, ChatCompletionRequest},
    common::GPT4_O,
};
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    pub objectives: Vec<String>,
    pub motivations: Vec<String>,
    pub training: Vec<String>,
    pub diet: Vec<String>,
    pub observations: Vec<String>,
    pub tips: Vec<String>,
    pub considerations: Vec<String>,
}

struct Deps {
    openai_client: OpenAIClient,
}

#[get("/ws")]
fn plan_stream(ws: rocket_ws::WebSocket, state: &State<Deps>) -> rocket_ws::Stream!['_] {
    let ws = ws.config(rocket_ws::Config {
        max_send_queue: Some(5),
        ..Default::default()
    });

    // let params = CreatePlanDto {
    //     weight: 86.0,
    //     height: 1.70,
    //     weight_goal: 75.0,
    //     hours_per_day: muscle_booster::HoursPerDay::Two,
    //     muscle_goal: muscle_booster::MuscleGoal::Gain,
    //     training_type: muscle_booster::TrainingType::Gym,
    //     motivations: vec![
    //         muscle_booster::Motivation::Healthy,
    //         muscle_booster::Motivation::Aesthetic,
    //     ],
    // };

    // let result = client.chat_completion(req).await.unwrap();

    // println!("{:#?}", result.choices[0].message.content);

    rocket_ws::Stream! {ws=>{
        for await message in  ws {
            match message {
                Ok(message) => {
                    let params: CreatePlanDto = serde_json::from_str(&message.to_string()).unwrap();
                    let req = ChatCompletionRequest::new(GPT4_O.to_string(), vec![
                        chat_completion::ChatCompletionMessage{
                            role: chat_completion::MessageRole::system,
                            content: chat_completion::Content::Text(r#"
                            Você irá responder em português e deverá criar um plano de treino baseado nos dados enviados pelo usuário. O formato da resposta deve ser um json sem MARKDOWN.
                            Estrutura:
                            {
                                "objectives": ["Objetivo 1", "Objetivo 2"],
                                "motivations": ["Motivação 1", "Motivação 2"],
                                "training": ["Treino 1", "Treino 2"],
                                "diet": ["Dieta 1", "Dieta 2"],
                                "observations": ["Observação 1", "Observação 2"],
                                "tips": ["Dica 1", "Dica 2"],
                                "considerations": ["Consideração 1", "Consideração 2"]
                            }
                            Quero no mínimo 4 itens em cada campo.
                            Seja específico no campo treinos, exemplo: "Treino 1: 3 séries de 10 repetições de supino reto com 50kg", "Treino 2: 4 séries de 12 repetições de rosca direta com 10kg".
                            "#.to_string()),
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

                    let result = state.openai_client.chat_completion(req).await.unwrap();
                    let content = result.choices[0].message.content.to_owned().unwrap().to_string();
                    let response: Response = serde_json::from_str(&content).unwrap();
                    let response = serde_json::to_string(&response).unwrap();


                    yield rocket_ws::Message::Text(response)
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }

        }
    }}
}

#[launch]
fn rocket() -> _ {
    let client = OpenAIClient::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let deps = Deps {
        openai_client: client,
    };
    rocket::build()
        .mount("/", routes![plan_stream])
        .manage(deps)
}
