#[macro_use] extern crate rocket;
use uuid::Uuid;
use clap::Parser;
use rocket::State;

// CLAP command line handling...
/// The amazingly simple Rocket daemon
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

// Structure used to store state shared between handlers...
struct DaemonState {
    uuid: Uuid,
}

#[get("/count")]
fn index(hit_count: &State<DaemonState>) -> String {
    format!("My id is {}", hit_count.uuid)
}

#[get("/users")]
fn get_users() -> &'static str {
    "You're my first user"
}

#[rocket::main]
async fn main() {
    let args = Args::parse(); 
    // Start Rocket using a custom ip address and port number
    let figment = rocket::Config::figment().merge(("port", args.port))
                                                    .merge(("address", args.address));
    rocket::custom(figment)
            .mount("/", routes![index, get_users])
            .manage(DaemonState { uuid: Uuid::new_v4(), })
            .launch().await;
}