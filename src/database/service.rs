use std::path::Path;
use warp::Filter;

async fn launch_server () {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!\n", name));

    warp::serve(hello)
    .run(([127, 0, 0, 1], 3030))
    .await;
}


#[tokio::main(worker_threads=128)]
pub async fn run_service(_database_dir: &Path, address: &str, port: i16) -> Result<(), String> {
    println!("Launching StateDB on address {} and port {}", address, port);

    launch_server().await;

    Ok(())
}