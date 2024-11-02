package main

import (
	"context"
	"fmt"
	"log"
	"net"

	dp "server/disciplina"
	pb "server/proto"

	"google.golang.org/grpc"
)

type server struct {
	pb.UnimplementedClienteServiceServer
}

func (s *server) EnviarCliente(ctx context.Context, req *pb.ClienteRequest) (*pb.ClienteResponse, error) {
	// Canal para recibir el resultado
	resultChan := make(chan string)

	// Ejecutar el envío en una goroutine
	go enviarAlumnoADisciplina(req, resultChan)

	// Esperar el resultado del canal sin timeout
	resultado := <-resultChan
	log.Printf("Respuesta en EnviarCliente server: %s", resultado)
	return &pb.ClienteResponse{
		Mensaje: resultado,
	}, nil
}

func enviarAlumnoADisciplina(alumno *pb.ClienteRequest, resultChan chan<- string) {
	// Conectar al servidor gRPC de AplicacionDisciplinas
	var conn *grpc.ClientConn
	var err error

	switch alumno.Disciplina {
	case 1:
		log.Printf("Conectando a disciplina: Natacion")
		conn, err = grpc.Dial("disciplina-natacion:50052", grpc.WithInsecure())
	case 2:
		log.Printf("Conectando a disciplina: Atletismo")
		conn, err = grpc.Dial("disciplina-atletismo:50053", grpc.WithInsecure())
	case 3:
		log.Printf("Conectando a disciplina: Boxeo")
		conn, err = grpc.Dial("disciplina-boxeo:50054", grpc.WithInsecure())
	default:
		log.Printf("Disciplina no encontrada para ID: %d", alumno.Disciplina)
		resultChan <- fmt.Sprintf("Disciplina no encontrada para ID: %d", alumno.Disciplina)
		return
	}

	// Manejo de error de conexión
	if err != nil {
		resultChan <- fmt.Sprintf("No se pudo conectar a AplicacionDisciplinas: %v", err)
		return
	}
	defer conn.Close()

	client := dp.NewDisciplinaServiceClient(conn)

	// Crear la solicitud gRPC para AplicacionDisciplinas
	req := &dp.AlumnoRequest{
		Nombre:     alumno.Nombre,
		Edad:       int32(alumno.Edad),
		Facultad:   alumno.Facultad,
		Disciplina: alumno.Disciplina,
	}

	// Usar contexto sin timeout
	ctx := context.Background()

	// Enviar la solicitud gRPC al servidor de AplicacionDisciplinas
	log.Println("Enviando solicitud gRPC a AplicacionDisciplinas...")
	resp, err := client.ProcesarAlumno(ctx, req)
	if err != nil {
		log.Printf("Error al enviar el alumno a AplicacionDisciplinas: %v", err)
		resultChan <- fmt.Sprintf("Error al enviar el alumno a AplicacionDisciplinas: %v", err)
		return
	}

	log.Printf("Respuesta de AplicacionDisciplinas: %s", resp.Mensaje)
	resultChan <- resp.Mensaje
}

func main() {
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("Error al iniciar el servidor: %v", err)
	}

	s := grpc.NewServer()
	pb.RegisterClienteServiceServer(s, &server{})

	log.Printf("Servidor de AplicacionPrincipal escuchando en el puerto 50051")
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Error al servir: %v", err)
	}
}
