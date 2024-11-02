package main

import (
	"context"
	"encoding/json"
	"log"

	"github.com/confluentinc/confluent-kafka-go/kafka"
	"github.com/go-redis/redis/v8"
)

var ctx = context.Background()

type Alumno struct {
	Nombre     string `json:"nombre"`
	Edad       int32  `json:"edad"`
	Facultad   string `json:"facultad"`
	Disciplina int32  `json:"disciplina"`
}

func main() {
	// Configura Redis
	rdb := redis.NewClient(&redis.Options{
		Addr:     "redis-master:6379",
		Password: "so2py2",
		DB:       0,
	})

	// Configura Kafka consumer
	c, err := kafka.NewConsumer(&kafka.ConfigMap{
		"bootstrap.servers": "my-cluster-kafka-bootstrap.default.svc.cluster.local:9092",
		"group.id":          "ganadores-group",
		"auto.offset.reset": "earliest",
	})
	if err != nil {
		log.Fatalf("Error al crear el consumidor de Kafka: %v", err)
	}

	c.SubscribeTopics([]string{"winners"}, nil)

	for {
		msg, err := c.ReadMessage(-1)
		if err == nil {
			var alumno Alumno

			// Deserializa el JSON
			err := json.Unmarshal(msg.Value, &alumno)
			if err != nil {
				log.Printf("Error al deserializar el mensaje JSON: %v", err)
				continue
			}

			// Actualiza los contadores en Redis
			if alumno.Facultad == "Agronomia" {
				rdb.HIncrBy(ctx, "facultades", "contador-agronomia", 1)
			} else if alumno.Facultad == "Ingenieria" {
				rdb.HIncrBy(ctx, "facultades", "contador-ingenieria", 1)
			}

			// Incrementa el contador de la disciplina del ganador
			switch alumno.Disciplina {
			case 1:
				rdb.HIncrBy(ctx, "ganadores-disciplinas", "contador-ganadores-natacion", 1)
			case 2:
				rdb.HIncrBy(ctx, "ganadores-disciplinas", "contador-ganadores-atletismo", 1)
			case 3:
				rdb.HIncrBy(ctx, "ganadores-disciplinas", "contador-ganadores-boxeo", 1)
			}

			log.Printf("Ganador procesado: %s, Facultad: %s, Disciplina: %d", alumno.Nombre, alumno.Facultad, alumno.Disciplina)
		} else {
			log.Printf("Error al recibir mensaje: %v", err)
		}
	}
}
