#![feature(decl_macro)]

extern crate postgres;
extern crate serde_json;
#[macro_use]extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

use postgres::NoTls;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, Deserialize)]
struct HostInfomation {
    host_id: i32,
    host_name: String,
    enabled: bool,
    mgr_set_name: String,
}



#[get("/host_info")]
fn host_info() -> Json<Vec<HostInfomation>>
{
    let mut vec = Vec::new();

    let mut conn = postgres::Client::connect("postgresql://ontune:ontune@192.168.0.188:5432/dashboard", NoTls).expect("failed connect");

     for row in &conn.query("select id, hostname, enabled, mgrsetname from board_hosts",&[]).unwrap() {
         let host_info = HostInfomation {
             host_id: row.get(0),
             host_name: row.get(1),
             enabled: row.get(2),
             mgr_set_name: row.get(3),
         };
         println!("Found Host Infomation {}", &host_info.host_name );
         vec.push(host_info);
     }
    Json(vec)
}

fn main() 
{
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![host_info])
        .launch();
}

