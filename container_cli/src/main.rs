#[macro_use] extern crate rocket;
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
    // Start Rocket using our custom port number
    let figment = rocket::Config::figment().merge(("port", args.port));
    rocket::custom(figment).mount("/", routes![index, get_users]).launch().await;
}