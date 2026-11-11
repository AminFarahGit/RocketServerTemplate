#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::serde::Deserialize;
use std::sync::Mutex;
use rocket::State;
use rocket::http::Status;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Response {
    id: usize,
    message: String,
}

struct StateData {
    responses: Mutex<Vec<Response>>,
    next_id: Mutex<usize>
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewResponse{
     message: String
}

#[get("/")]
fn index() -> &'static str {
    "Server is now operational"
}

#[get("/path/<id>")]
fn get_response_from_id(id: usize, state_data: &State<StateData>) -> Result<Json<Vec<Response>>, Status> {
    let responses = state_data.responses.lock().unwrap();
    let response = responses.iter().find(|response| response.id == id);
    match response {
        Some(response) => Ok(Json(vec![response.clone()])),
        None => Err(Status::NotFound),
    }
}

#[get("/path/<first_param>/<second_param>")]
fn path(first_param: String, second_param: u8) -> String {
    format!("First param: {}, Second param: {}", first_param, second_param)
}

#[get("/path")]
fn default_path(state_data: &State<StateData>) -> Json<Vec<Response>>{
    let responses = state_data.responses.lock().unwrap();
    Json(responses.clone())
}

#[post("/path", data="<new_response>")]
fn update_state(new_response: Json<NewResponse>, state_data: &State<StateData>) -> Json<Response>{
    let mut next_id = state_data.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;


    //Unwrap json
    let incoming = new_response.into_inner();

    let response = Response {
        id,
        message: incoming.message,
    };

    state_data.responses.lock().unwrap().push(response.clone());
    Json(response)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(StateData {
            responses: Mutex::new(Vec::new()),
            next_id: Mutex::new(0)
        })
        .mount("/", routes![index, path, default_path, update_state, get_response_from_id])
}
