use axum::{Server, Router, routing::get, extract::State};
use sysinfo::{CpuExt, System, SystemExt};
use std::sync::{Arc, Mutex };


#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root_get))
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


async fn root_get(State(state): State<AppState>) -> String{
    use std::fmt::Write;
    let mut s = String::new(); 
    let mut sys = state.sys.lock().unwrap();

    sys.refresh_cpu();

    for (i,cpu) in sys.cpus().iter().enumerate(){
        let i = i+1;
        let usage = cpu.cpu_usage();
        writeln!(&mut s,"CPU {i} {usage}%" ).unwrap();
    }


    s
}