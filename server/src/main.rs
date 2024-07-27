pub mod models;
pub mod schema;

use ntex::web::{ App,
                 HttpServer,
                 HttpResponse,
                 Responder,
                 get,
                 post };
use ntex::web;
use diesel::prelude::*;
use server::establish_connection;
use serde::{ Serialize,
             Deserialize };

#[derive(Serialize, Deserialize)]
pub struct ClientPaymentStatusRequest {
    pub pw: String,
    pub target_gid: i32,
    pub set_paid: bool
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hai")
}

#[post("/clients/{id}/register")]
pub async fn register_client(path: web::types::Path<i32>) -> impl Responder {
    use self::models::NewClient;
    use crate::schema::clients;

    let connection = &mut establish_connection();
    let new_client = NewClient { gid: *path };

    diesel::insert_into(clients::table)
        .values(&new_client)
        .returning(self::models::Client::as_returning())
        .get_result(connection)
        .expect("Error registering client");

    HttpResponse::Ok().body("OK")

}

#[get("/client/{id}/status")]
pub async fn check_client_status(path: web::types::Path<i32>) -> impl Responder {
    use self::schema::clients::dsl::*;
    use self::models::*;
    
    let req_gid: i32 = path.clone();
    let connection = &mut establish_connection();
    let results: Vec<Client> = match clients
        .filter(gid.eq(req_gid))
        .limit(1)
        .load(connection) {
            Ok(vec) => vec,
            Err(..) => {
                return HttpResponse::NotFound().body("Not Found");
            }
        };

    let ispaid = results[0].paid;
    
    HttpResponse::Ok().body(format!("Paid: {}", ispaid))
}

#[post("/admin/client/setstatus")]
pub async fn set_client_status(body: ntex::web::types::Json<ClientPaymentStatusRequest>) -> impl Responder {
    use crate::schema::clients::dsl::clients;
    use crate::schema::clients::{gid, paid};
    use crate::models::Client;

    let pw = "ToBeReplacedByBuildScript";
    if body.pw == pw {
        let connection = &mut establish_connection();
        let results = diesel::update(&clients.filter(gid.eq(body.target_gid))
            .first::<Client>(connection)
            .expect("Client not found"))
            .set(paid.eq(body.set_paid))
            .get_result::<Client>(connection)
            .expect("Could not update Client");

        return HttpResponse::Ok().json(&results);
    } else {
        return HttpResponse::Unauthorized().body("Invalid Password")
    }
}


#[ntex::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(register_client)
            .service(check_client_status)
            .service(set_client_status)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
