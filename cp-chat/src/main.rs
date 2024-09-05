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
    prelude::*,    //imports traits and types to interact with databasection pooling library, efficiently manage database connections in web application
    r2d2::{self, ConnectionManager}, 
};
//self means current module that you want to bring into the scope
//ConnectionManager is struct used to manage DB connection in Diesel

mod db;    //handles database connection and related operations
mod models;    //place to keep data models for application representing database tables and relationships
mod routes;    //routing logic handling of the web server
mod schema;    //table definitions
mod server;    //logic for running WebSocket or HTTP servers
mod session;    //user sessions or state management for WebSocket or HTTP server