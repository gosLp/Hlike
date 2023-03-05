use axum::{
    Server,
    Router,
    routing::get,
    extract::{State, WebSocketUpgrade, ws::{WebSocket, Message}},
    Json,
    response::{IntoResponse, Html}, http::Response};
use sysinfo::{CpuExt, System, SystemExt};
use std::sync::{Arc, Mutex };
use tokio::sync::broadcast;


type Snapshot = Vec<f32>;

#[tokio::main]
async fn main() {

    let (tx, _)= broadcast::channel::<Snapshot>(10);

    let app_state = AppState{
        tx: tx.clone()
    };

    let app = Router::new()
        .route("/", get(root_get))
        // .route("/api/cpus", get(cpus_get))
        .route("/index.mjs", get(indexmjs_get))
        .route("/index.css",  get(indexcss_get))
        .route("/realtime/cpus", get(realtime_cpus_get))
        .with_state(app_state.clone());


    let app_state_for_bg = app_state.clone();


    //Update CPU usage in the Background
    tokio::task::spawn_blocking(move ||{
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            let _ =tx.send(v);

            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    let server = Server::bind(&"0.0.0.0:7832".parse().unwrap())
        .serve(app.into_make_service());

    let local_addr= server.local_addr();
    println!("Listening in on {local_addr}");
    server.await.unwrap();
}

#[derive(Clone)]
struct AppState{
    tx: broadcast::Sender<Snapshot>,

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
async fn indexcss_get() -> impl IntoResponse{
   let markup = tokio::fs::read_to_string("src/index.css").await.unwrap();
   Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

// #[axum::debug_handler]
// async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse{
//     // use std::fmt::Write;
//     // let mut s = String::new(); 

//     let lock_start = std::time::Instant::now();

//     let v = state.cpus.lock().unwrap().clone();
//     let lock_elapsed = lock_start.elapsed().as_millis();
//     // println!("Lock time: {:?}ms",lock_elapsed);

//     // for (i,cpu) in sys.cpus().iter().enumerate(){
//     //     let i = i+1;
//     //     let usage = cpu.cpu_usage();
//     //     writeln!(&mut s,"CPU {i} {usage}%" ).unwrap();
//     // }


//     Json(v)
// }

#[axum::debug_handler]
async fn realtime_cpus_get(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
)-> impl IntoResponse{
    ws.on_upgrade(|ws: WebSocket| async {
        realtime_cpus_stream(state, ws).await
    })
}

async fn realtime_cpus_stream(app_state: AppState,mut ws:WebSocket) {

    let mut rx = app_state.tx.subscribe();
    // let cpus = app_state.cpus.lock().unwrap().clone();

    while let Ok(msg)= rx.recv().await{
        
        ws.send(Message::Text(serde_json::to_string(&msg).unwrap()))
        .await
        .unwrap();


    }
    

}