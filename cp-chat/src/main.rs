//WebSocket Server
//handles communication between the clients

#[macro_use] extern crate diesel; //imports diesel crate

use actix::*;   //imports Actix framework for web servers
use actix_cors::Cors;   //CORS:Cross-Origin Resource Sharing, useful when server and clients on different domains
use actix_files::Files;    //imports files like HTML, CSS, JS directly from server
use actix_web::{web, http, App, HttpServer};
//web: for configuring routes and extracting data
//http: for working with HTTP types and utils
//app: represents application where routes and middlewares are confirmed
//httpserver: actual server that will run the app

use diesel::{
    prelude::*, //imports traits and types to interact with database
    r2d2::{seld, ConnectionManager},    //connection pooling library, efficiently manage database connections in web application
};

mod db;
mod models;
mod routes;
mod schema;
mod server;
mod session;