use std::net::SocketAddr;
use std::sync::mpsc;
use std::thread;
use tonic::{transport::Server, Request, Response, Status};
use proto::cliente_service_server::{ClienteService, ClienteServiceServer};
use proto::{ClienteRequest, ClienteResponse};
use disciplina_proto::disciplina_service_client::DisciplinaServiceClient;
use disciplina_proto::AlumnoRequest;

pub mod proto {
    tonic::include_proto!("cliente"); // Compila `cliente.proto`
}

pub mod disciplina_proto {
    tonic::include_proto!("disciplina"); // Compila `disciplina.proto`
}

#[derive(Debug, Default)]
pub struct MyClienteService {}

#[tonic::async_trait]
impl ClienteService for MyClienteService {
    async fn enviar_cliente(
        &self,
        request: Request<ClienteRequest>,
    ) -> Result<Response<ClienteResponse>, Status> {
        let alumno = request.into_inner();
        
        // Creamos un canal de comunicación entre el hilo y la función principal
        let (tx, rx) = mpsc::channel();

        // Lanzamos un hilo para enviar el alumno a la disciplina correspondiente
        thread::spawn(move || {
            let result = enviar_alumno_a_disciplina(alumno);
            if let Err(e) = tx.send(result) {
                eprintln!("Error enviando resultado al canal: {:?}", e);
            }
        });

        // Esperamos el resultado del hilo a través del canal
        let resultado = rx.recv().unwrap_or_else(|_| "Error en procesamiento".to_string());
        Ok(Response::new(ClienteResponse { mensaje: resultado }))
    }
}

fn enviar_alumno_a_disciplina(alumno: ClienteRequest) -> String {

    let disciplina_addr = match alumno.disciplina {
        1 => "http://disciplina-natacion:50052",
        2 => "http://disciplina-atletismo:50053",
        3 => "http://disciplina-boxeo:50054",
        _ => return format!("Disciplina no encontrada para ID: {}", alumno.disciplina),
    };

    //imprimir en consola la disciplina a la que se va a enviar el alumno y la información del alumno
    println!("Alumno: {:?}", alumno);

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let mut client = match DisciplinaServiceClient::connect(disciplina_addr).await {
            Ok(client) => client,
            Err(e) => return format!("Error al conectar con la disciplina: {:?}", e),
        };

        let request = tonic::Request::new(AlumnoRequest {
            nombre: alumno.nombre,
            edad: alumno.edad,
            facultad: alumno.facultad,
            disciplina: alumno.disciplina,
        });

        match client.procesar_alumno(request).await {
            Ok(response) => response.into_inner().mensaje,
            Err(e) => format!("Error al procesar alumno en disciplina: {:?}", e),
        }
    });

    //prueba retornar ok para ver si se envia el alumno a la disciplina
    result

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "0.0.0.0:50051".parse().unwrap();
    let cliente_service = MyClienteService::default();

    println!("Servidor de AplicacionPrincipal escuchando en el puerto 50051");

    Server::builder()
        .add_service(ClienteServiceServer::new(cliente_service))
        .serve(addr)
        .await?;

    Ok(())
}
