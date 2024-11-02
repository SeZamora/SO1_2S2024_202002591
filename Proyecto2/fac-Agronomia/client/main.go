package main

import (
	"context"
	"fmt"
	"log"

	pb "client/proto"

	"github.com/gofiber/fiber/v2"
	"google.golang.org/grpc"
)

type ClienteRequest struct {
	Nombre     string `json:"nombre"`
	Edad       int    `json:"edad"`
	Facultad   string `json:"facultad"`
	Disciplina int    `json:"disciplina"`
}

func main() {
	// Inicializa Fiber
	app := fiber.New()

	app.Post("/agronomia", func(c *fiber.Ctx) error {
		// Decodificar el JSON recibido
		var cliente ClienteRequest
		if err := c.BodyParser(&cliente); err != nil {
			log.Printf("Error al decodificar el JSON: %v", err)
			return c.Status(fiber.StatusBadRequest).SendString("Error al decodificar el JSON")
		}

		// Conectar al servidor gRPC
		log.Println("Conectando al servidor gRPC en localhost:50051...")
		conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
		if err != nil {
			log.Printf("No se pudo conectar al servidor gRPC: %v", err)
			return c.Status(fiber.StatusInternalServerError).SendString(fmt.Sprintf("No se pudo conectar al servidor gRPC: %v", err))
		}
		defer conn.Close()

		client := pb.NewClienteServiceClient(conn)

		// Crear la solicitud gRPC
		req := &pb.ClienteRequest{
			Nombre:     cliente.Nombre,
			Edad:       int32(cliente.Edad),
			Facultad:   cliente.Facultad,
			Disciplina: int32(cliente.Disciplina),
		}

		ctx := context.Background()

		log.Printf("Enviando solicitud gRPC para el alumno: %s", cliente.Nombre)
		resp, err := client.EnviarCliente(ctx, req)
		if err != nil {
			log.Printf("Error al enviar la solicitud gRPC: %v", err)
			return c.Status(fiber.StatusInternalServerError).SendString(fmt.Sprintf("Error al enviar la solicitud gRPC: %v", err))
		}

		// Retornar la respuesta del servidor gRPC al cliente HTTP
		log.Printf("Respuesta recibida del servidor gRPC: %s", resp.Mensaje)
		return c.SendString(resp.Mensaje)
	})

	// Iniciar el servidor HTTP en el puerto 8080
	log.Println("Cliente HTTP escuchando en el puerto :8080")
	log.Fatal(app.Listen(":8080"))
}
