package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
	"time"

	pb "agronomia"

	"google.golang.org/grpc"
)

// Definir estructura del alumno
type Alumno struct {
	Nombre     string `json:"nombre"`
	Edad       int32  `json:"edad"`
	Facultad   string `json:"facultad"`
	Disciplina string `json:"disciplina"`
}

func enviarAlumno(alumno Alumno) {
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("Error al conectar con servidor de disciplinas: %v", err)
	}
	defer conn.Close()

	client := pb.NewDisciplinaServiceClient(conn)
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	req := &pb.AlumnoRequest{
		Nombre:     alumno.Nombre,
		Edad:       alumno.Edad, // Pasar el campo de edad
		Facultad:   alumno.Facultad,
		Disciplina: alumno.Disciplina,
	}

	_, err = client.EnviarAlumno(ctx, req)
	if err != nil {
		log.Fatalf("Error al enviar alumno: %v", err)
	}
}

func handler(w http.ResponseWriter, r *http.Request) {
	var alumno Alumno
	err := json.NewDecoder(r.Body).Decode(&alumno)
	if err != nil {
		http.Error(w, "Solicitud inválida", http.StatusBadRequest)
		return
	}

	// Enviar el alumno a las disciplinas
	enviarAlumno(alumno)

	w.Write([]byte("Alumno enviado a disciplina correctamente"))
}

func main() {
	http.HandleFunc("/enviar", handler)
	log.Println("Servidor de Ingeniería corriendo en puerto 8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
