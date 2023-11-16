mod sql_adapter;
pub use sql_adapter::{User, select_all_data};

use actix_web::{web::Data, App, HttpServer, get, Responder, HttpResponse};

use crate::AppState;

#[get("/hello")]
pub async fn hello_handler(state: Data<AppState>) -> impl Responder{
    let res = select_all_data(&state.db).await;

    match res {
        Ok(res) => {
            for i in res {
                println!("id: {}, name: {}, password: {}, info: {}", i.id, i.user_name, i.password, i.user_info)
            }
        }

        Err(e) => eprint!("Error printing rows: {}", e)
    }
    
    HttpResponse::Ok().body("hello, friend!")
}