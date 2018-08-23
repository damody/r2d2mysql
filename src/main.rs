extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate r2d2;
extern crate uuid;
extern crate bytes;
extern crate dotenv;
extern crate chrono;
extern crate r2d2_diesel;

use actix::prelude::*;
use actix_web::{
    error, http, middleware, server, App, AsyncResponder, Error, HttpMessage,
HttpRequest, HttpResponse,
};
use bytes::BytesMut;
use futures::{Future, Stream};
use diesel::prelude::*;
//use diesel::r2d2::ConnectionManager;

use r2d2_diesel::ConnectionManager;
use r2d2::Pool;

pub type DBPool = Pool<ConnectionManager<MysqlConnection>>;

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub DBPool);
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
#[derive(Debug)]
pub struct CreateMember {
    pub email: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub password: String,
    pub gender: i8,
}


impl Message for CreateMember {
    type Result = Result<(), Error>;
}
impl Handler<CreateMember> for DbExecutor {
    type Result = Result<(), Error>;
    fn handle(&mut self, msg: CreateMember, _: &mut Self::Context) -> Self::Result {
        Ok(())
    }
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    use std::env;
    let sys = actix::System::new("diesel-example");
    let _ = dotenv::dotenv();
    let url = env::var("MYSQL_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set in order to run unit tests");
    // Start 3 db executor actors
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = r2d2::Pool::new(manager)
        .expect("Failed to create pool.");
    pool.get().unwrap();
    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));
}
