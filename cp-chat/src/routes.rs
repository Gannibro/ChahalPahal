// src/routes.rs
use std::time::Instant;     //imports 'Instant' fro rust, used to measure elpsed time, or time to handle requests/connections
use actix::*;     //imports everything (*) from Actix framework, utils for creating and managing actors
use actix_files::NamedFile;     //imports 'NamedFile' used to serve static files like images, HTML, JS and CSS
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
/*get: macro to define GET routes
  post: macro to define POST routes
  web: utils for routing and extracting data from requests
  error: type that represents errors in web framework
  httpRequest: represents incoming HTTP request, used to access headers and query parameters
  httpResponse: represents http response that will be sent back to client
  responder: defines types that can generate HTTP response */
use actix_web_actors::ws;     //imporst WebSocket module(ws), that includes utils to handle messages, frames and connection upgrades
use diesel::{
    prelude::*,     //imports types to interact with databasection pooling library, efficiently manage database connections in web application
    r2d2::{self, ConnectionManager},
    //self means current module that you want to bring into the scope
    //ConnectionManager is struct used to manage DB connection in Diesel
};