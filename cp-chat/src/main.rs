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

#[actix_web::main]   //turns main into aasync function to setup Actix system
//async function allows a program to handle long running tasks without blocking main thread

async fn main() -> std::io::Result<()> {     //allows to start server and interact with DB
    let server = server::ChatServer::new().start();     //starts a ChatServer instance(WebSocket server)
    let conn_spec = "chat.db";     //string specifying location of SQLite DB
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);     //initializes 'ConnectionManager' for SQLite using Diesel ORM
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");    //allows server to reuse DB connections
    let server_addr = "127.0.0.1";     //localhost server IP
    let server_port = 8080;
    let app = HttpServer::new(move || {     //sets up HTTP server using Actix, 'move' allows captured variables to be used
        let cors = Cors::default()     //configuress CORS for the server
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")    //specified domains to make requests
            .allowed_methods(vec!["GET", "POST"])    //defines which HTTP methods are allowed
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])   //specifies which headers are allowed
            .allowed_header(http::header::CONTENT_TYPE)     //sets how long CORS preflight request can be cached
            .max_age(3600);
            /*A CORS preflight request is a request that checks if CORS protocol is understood and a server is aware 
            using specific methods and headers*/

        App::new()   //creates new instancce of Actix web app (container for HTTP routes and services)
            .app_data(web::Data::new(server.clone()))    //adds shared app state and server cloning to ensure object is accessible across multiple requests
            .app_data(web::Data::new(pool.clone()))     //adds DB connection pool and clones to enable DB interactions in a safe way
            .wrap(cors)     //applies CORS middleware to app, ensuring correct hadling of requests from front end
            .service(web::resource("/").to(routes::index))     //defines a route for the route path '/' (will be invoked once a client sends a request to root URL)
            .route("/ws", web::get().to(routes::chat_server))     //defines WebSocket route at /ws
            .service(routes::create_user)     //adds a route for creating a new user
            .service(routes::get_user_by_id)    //defines a route to fetch a user by their ID
            .service(routes::get_user_by_phone)     //defines a route to fetch a user by their phone number
            .service(routes::get_conversation_by_id)    //defines a route to fetch a conversation by its ID
            .service(routes::get_rooms)     //defines a route to fetch all available chat rooms 
            .service(Files::new("/", "./static"))     //configures the server to serve static files from (./static) directory and requests static resources
    })
    .workers(2)     //speifies that the server should run with 2 worker threads
    .bind((server_addr, server_port))?      //binds server to a specific address and port (Address: 127.0.0.1, Port: 8080)
    .run();     //starts HTTP server and listens for incoming requests
    println!("Server running at http://{server_addr}:{server_port}/");
    app.await     //waits for the server to complete its execution
}
/*App is setup to handle HTTP and WebSocket connections with routes for static files, handling user and messaging data.
  Shared resources like WebSocket server and DB connection pool are injected into the app,
  whilst CORS allows requests from speecific origins and server is bount to 127.0.0.1 : 8080 address */