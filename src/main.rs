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
                    let params: CreatePlanDto = serde_json::from_str(&message.into_text()?).unwrap();
                    let req = ChatCompletionRequest::new(GPT4_O.to_string(), vec![
                        chat_completion::ChatCompletionMessage{
                            role: chat_completion::MessageRole::system,
                            content: chat_completion::Content::Text(r#"
                            Você irá responder em português e deverá criar um plano de treino baseado nos dados enviados pelo usuário. O formato da resposta deve ser html sem MARKDOWN.
                            Irei usar tailwindcss para estilizar o html.
                            Estrutura:
                            div, classes="bg-white shadow-lg p-4 rounded-lg max-w-md"
                                h2 - Plano de treino, classes="text-2xl font-bold"
                                    div, classes="bg-white shadow-lg p-4 rounded-lg"
                                    h3 - Objetivos, classes="text-lg font-bold"
                                        ul, classes="list-disc pl-4"
                                            Itens sendo listados
                                    h3 - Motivações, classes="text-lg font-bold"
                                        ul, classes="list-disc pl-4"
                                            Itens sendo listados
                                    h3 - Treino, classes="text-lg font-bold"
                                        ul, classes="list-disc pl-4"
                                            Itens sendo listados
                                    h3 - Dieta, classes="text-lg font-bold"
                                        ul, classes="list-disc pl-4"
                                            Itens sendo listados
                                    h3 - Observações, classes="text-lg font-bold"
                                        ul, classes="list-disc pl-4"
                                            Itens sendo listados
                                    h3 - Dicas, classes="text-lg font-bold"
                                        ul, classes="list-disc pl-4"
                                            Itens sendo listados
                                    h3 - Considerações, classes="text-lg font-bold"
                                        ul, classes="list-disc pl-4"
                                            Itens sendo listados
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


                    yield rocket_ws::Message::Text(result.choices[0].message.content.to_owned().unwrap())
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
