#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use dotenv::dotenv;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod db;
pub mod models;
pub mod schema;

use models::{Employee, NewEmployee};
use schema::employees::dsl::*;

type DbPool = Arc<Mutex<db::Pool>>;

async fn get_employees(pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.lock().await;
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let results = employees
        .limit(10)
        .load::<Employee>(&mut conn)
        .expect("Error loading employees");

    HttpResponse::Ok().json(results)
}

async fn add_employee(pool: web::Data<DbPool>, item: web::Json<NewEmployee>) -> impl Responder {
    let pool = pool.lock().await;
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let new_employee = NewEmployee {
        name: item.name.clone(),
        age: item.age,
        position: item.position.clone(),
    };

    diesel::insert_into(employees)
        .values(&new_employee)
        .execute(&mut conn)
        .expect("Error saving new employee");

    HttpResponse::Ok().json("Employee added")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let pool = db::establish_connection();
    let pool = Arc::new(Mutex::new(pool));

    log::info!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/employees", web::get().to(get_employees))
            .route("/employees", web::post().to(add_employee))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
