use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
mod workerstate;
mod teststruct;

use workerstate::WorkerState;
use teststruct::TestStruct;

struct AppState{
    work_state: Mutex<WorkerState>,
}

struct TestingStruct{
    test_state: Mutex<TestStruct>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("You are connected to the server")
}

#[get("/start")]
async fn start_work(data: web::Data<AppState>) -> impl Responder {
    match data.work_state.lock().unwrap().start_work(){
        Ok(()) => HttpResponse::Ok().body("Success starting work thread"),
        Err(err) => HttpResponse::Ok().body(err),
    }
}

#[get("/end")]
async fn end_work(data: web::Data<AppState>) -> impl Responder {
    match  data.work_state.lock().unwrap().end_work(){
        Ok(()) => HttpResponse::Ok().body("Successfully ended the thread"),
        Err(err) => HttpResponse::Ok().body(err),
    }
}

#[get("/increment")]
async fn increment_struct(data: web::Data<TestingStruct>) -> impl Responder{
    data.test_state.lock().unwrap().up();
    let val = data.test_state.lock().unwrap().count;
    HttpResponse::Ok().body(format!("Wat {}", val))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let worker_state = web::Data::
        new(
            AppState{work_state : Mutex::new(WorkerState::new(23).unwrap())}
        );
    HttpServer::new(move || {
        App::new()
            .app_data(worker_state.clone())
            .data(TestingStruct{ test_state : Mutex::new(TestStruct{ count : 0})})
            .service(hello)
            .service(start_work)
            .service(end_work)
            .service(increment_struct)
    }).bind("0.0.0.0:8080")?
    .run()
    .await
}
