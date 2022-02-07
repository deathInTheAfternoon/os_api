#[macro_use] extern crate rocket;
use std::net::{Ipv4Addr, IpAddr};
use uuid::Uuid;
use clap::Parser;

// The amazingly simple Rocket daemon
#[derive(Parser, Debug)]
#[clap(name("Rocket daemon"), author("DeathInTheAfternoon"), version("0.1"), about("Does what is says on the tin."), long_about=None)]
struct Args {
    /// Set the listening port
    /// 
    /// This parameter will configure the port on which this daemon listens for incoming requests.
    #[clap(short('p'), long("port"), default_value_t = 8080)]
    port: i32,

    /// Set host IP to which we bind
    /// 
    /// When in Docker container bind to ALL interfaces 0.0.0.0 NOT 127.0.0.1.
    #[clap(short('a'), long("address"), default_value = "127.0.0.1")]
    address: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello Naveen"
}

#[get("/users")]
fn get_users() -> &'static str {
    "You're my first user"
}

#[rocket::main]
async fn main() {
    println!("Starting server with uuid {}",Uuid::new_v4());
    let args = Args::parse(); 
    // Start Rocket using a custom ip address and port number
    let figment = rocket::Config::figment().merge(("port", args.port))
                                                    .merge(("address", args.address));
    rocket::custom(figment).mount("/", routes![index, get_users]).launch().await;
}