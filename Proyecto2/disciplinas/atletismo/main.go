package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"net"
	"time"

	"github.com/confluentinc/confluent-kafka-go/kafka"
	"google.golang.org/grpc"

	dp "atletismo/disciplina"
)

type server struct {
	dp.UnimplementedDisciplinaServiceServer
	producer *kafka.Producer
}

// ProcesarAlumno maneja la solicitud gRPC y decide si el alumno es ganador o perdedor
func (s *server) ProcesarAlumno(ctx context.Context, req *dp.AlumnoRequest) (*dp.AlumnoResponse, error) {
	// Decidir si el alumno es ganador o perdedor
	resultado := determinarGanador(req.Disciplina)

	log.Printf("Procesando alumno: %s. Resultado: %s", req.Nombre, resultado)

	// Elegir el topic basado en el resultado
	topic := "losers"
	if resultado == "ganador" {
		topic = "winners"
	}

	// Crear el mensaje a enviar, incluyendo el resultado del alumno
	alumnoConResultado := struct {
		Nombre     string `json:"nombre"`
		Edad       int32  `json:"edad"`
		Facultad   string `json:"facultad"`
		Disciplina int32  `json:"disciplina"`
	}{
		Nombre:     req.Nombre,
		Edad:       req.Edad,
		Facultad:   req.Facultad,
		Disciplina: req.Disciplina,
	}

	// Serializar el mensaje en formato JSON
	alumnoJSON, err := json.Marshal(alumnoConResultado)
	if err != nil {
		log.Printf("Error al serializar el alumno a JSON: %v", err)
		return nil, fmt.Errorf("error al serializar el alumno a JSON: %v", err)
	}

	// Aumentar el timeout para dar más tiempo en la operación gRPC
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// Enviar el mensaje JSON a Kafka y medir el tiempo de procesamiento
	start := time.Now()
	log.Println("Iniciando envío de mensaje a Kafka...")

	err = enviarMensajeKafka(s.producer, topic, alumnoJSON)
	if err != nil {
		log.Printf("Error al enviar mensaje a Kafka: %v", err)
		return nil, fmt.Errorf("error al enviar mensaje a Kafka: %v", err)
	}

	log.Printf("Mensaje enviado a Kafka en %v", time.Since(start))

	// Retornar el mensaje de respuesta al cliente gRPC
	return &dp.AlumnoResponse{
		Mensaje: fmt.Sprintf("El alumno %s ha sido procesado y es %s", req.Nombre, resultado),
	}, nil
}

// determinarGanador lanza una moneda para decidir el ganador o perdedor
func determinarGanador(disciplina int32) string {
	if rand.Intn(2) == 0 {
		return "ganador"
	}
	return "perdedor"
}

// enviarMensajeKafka envía el mensaje a Kafka en formato JSON
func enviarMensajeKafka(producer *kafka.Producer, topic string, mensaje []byte) error {
	// Publicar el mensaje en Kafka
	err := producer.Produce(&kafka.Message{
		TopicPartition: kafka.TopicPartition{Topic: &topic, Partition: kafka.PartitionAny},
		Value:          mensaje,
	}, nil)

	if err != nil {
		log.Printf("Error al producir el mensaje: %v", err)
		return err
	}

	// Asegurar que se enviaron todos los mensajes
	flushTimeout := 5000 // Tiempo de espera para flush en milisegundos (15 segundos)
	remainingMessages := producer.Flush(flushTimeout)
	if remainingMessages > 0 {
		log.Printf("Atletismo No se pudieron enviar todos los mensajes. Mensajes restantes: %d", remainingMessages)
		return fmt.Errorf("Atletismo No se pudieron enviar todos los mensajes. Mensajes restantes: %d", remainingMessages)
	}

	return nil
}

func main() {
	// Crear el productor de Kafka conectado al clúster en el namespace `default`
	p, err := kafka.NewProducer(&kafka.ConfigMap{
		"bootstrap.servers": "my-cluster-kafka-bootstrap.default.svc.cluster.local:9092",
	})
	if err != nil {
		log.Fatalf("Error al crear el productor de Kafka: %v", err)
	}
	defer p.Close()

	// Iniciar el servidor gRPC
	lis, err := net.Listen("tcp", ":50053")
	if err != nil {
		log.Fatalf("Error al iniciar el servidor: %v", err)
	}

	s := grpc.NewServer()
	dp.RegisterDisciplinaServiceServer(s, &server{producer: p})

	log.Printf("Servidor de Atletismo escuchando en el puerto 50053")
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Error al servir: %v", err)
	}
}
