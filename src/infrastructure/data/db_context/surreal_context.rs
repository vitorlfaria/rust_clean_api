use once_cell::sync::Lazy;
use surrealdb::{Surreal, engine::remote::ws::{Client, Ws}, opt::auth::Root};


pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn connect_db() {
    let _ = DB.connect::<Ws>("localhost:8000").await;
    let _ = DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await;
    let _ = DB.use_ns("todo").use_db("todo").await;
}
