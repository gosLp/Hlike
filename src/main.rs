use axum::{Server, Router, routing::get, extract::State, Json, response::{IntoResponse, Html}, http::Response};
use sysinfo::{CpuExt, System, SystemExt};
use std::sync::{Arc, Mutex };


#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root_get))
        .route("/api/cpus", get(cpus_get))
        .route("/index.mjs", get(indexmjs_get))
        .with_state(AppState{
            sys: Arc::new(Mutex::new(System::new())),
        });
    let server = Server::bind(&"0.0.0.0:7832".parse().unwrap())
        .serve(app.into_make_service());

    let local_addr= server.local_addr();
    println!("Listening in on {local_addr}");
    server.await.unwrap();
}

#[derive(Clone)]
struct AppState{
    sys: Arc<Mutex<System>>,

}

#[axum::debug_handler]
async fn root_get() -> impl IntoResponse{
   let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
   Html(markup)
}


#[axum::debug_handler]
async fn indexmjs_get() -> impl IntoResponse{
   let markup = tokio::fs::read_to_string("src/index.mjs").await.unwrap();
   Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}


#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse{
    // use std::fmt::Write;
    // let mut s = String::new(); 
    
    let mut sys = state.sys.lock().unwrap();

    //FIXME: this blocks, yes i feel bad. Later. also find out what blocking in rust works
    sys.refresh_cpu();
    let v_cpus: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    // for (i,cpu) in sys.cpus().iter().enumerate(){
    //     let i = i+1;
    //     let usage = cpu.cpu_usage();
    //     writeln!(&mut s,"CPU {i} {usage}%" ).unwrap();
    // }


    Json(v_cpus)
}