use std::io;

use rocket::fs::FileServer;
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};

#[macro_use]
extern crate rocket;

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![delay, blocking_task])
        .mount("/public", FileServer::from("static/"))
}
