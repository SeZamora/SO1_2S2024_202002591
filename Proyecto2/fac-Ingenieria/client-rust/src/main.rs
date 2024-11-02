use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tonic::Request;
use proto::cliente_service_client::ClienteServiceClient;
use proto::ClienteRequest;

pub mod proto {
    tonic::include_proto!("cliente"); // Esto incluye el archivo `cliente.proto` compilado
}

#[derive(Deserialize, Serialize)]
struct ClienteData {
    nombre: String,
    edad: i32,
    facultad: String,
    disciplina: i32,
}

async fn handle_cliente(cliente: web::Json<ClienteData>) -> impl Responder {
    // Conectar al servidor gRPC
    let mut client = match ClienteServiceClient::connect("http://localhost:50051").await {
        Ok(client) => client,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to connect to gRPC server: {}", e)),
    };

    // Crear solicitud gRPC
    let request = Request::new(ClienteRequest {
        nombre: cliente.nombre.clone(),
        edad: cliente.edad,
        facultad: cliente.facultad.clone(),
        disciplina: cliente.disciplina,
    });

    // Enviar la solicitud y manejar la respuesta
    match client.enviar_cliente(request).await {
        Ok(response) => {
            let mensaje = response.into_inner().mensaje;
            HttpResponse::Ok().json(format!("Respuesta del servidor: {}", mensaje))
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("gRPC call failed: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .route("/ingenieria", web::post().to(handle_cliente))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
