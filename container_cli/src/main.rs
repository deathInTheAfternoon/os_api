#[macro_use] extern crate rocket;
use uuid::Uuid;
use clap::Parser;
use rocket::State;

// CLAP command line handling...
/// Our amazingly simple Rocket daemon
#[derive(Parser, Debug)]
#[clap(name("Rocket daemon"), author("DeathInTheAfternoon"), version("0.1"), about("Does what is says on the tin."), long_about=None)]
struct Args {
    /// Set the port on which we will listen
    /// 
    /// This parameter will configure the port on which this daemon listens for incoming requests.
    #[clap(short('p'), long("port"), default_value_t = 8080)]
    port: i32,

    /// Set bind IP on which we will listen
    /// 
    /// When in container bind to ALL interfaces 0.0.0.0
    #[clap(short('a'), long("address"), default_value = "127.0.0.1")]
    address: String,
}

// Structure used to store state shared between Rocket handlers...
struct DaemonState {
    uuid: Uuid,
}

#[get("/")]
fn index(hit_count: &State<DaemonState>) -> String {
    format!("My id is {}", hit_count.uuid)
}

#[rocket::main]
async fn main() {
    // Parse command line arguments using Clap
    let args = Args::parse(); 
    // Start Rocket using the custom ip address and port number, if any
    let figment = rocket::Config::figment().merge(("port", args.port))
                                                    .merge(("address", args.address));
    rocket::custom(figment)
            .mount("/", routes![index])
            .manage(DaemonState { uuid: Uuid::new_v4(), }) // Generate a uuid for this instance
            .launch().await;
}