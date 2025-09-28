package main

import (
	"context"
	"encoding/json"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/milvus-io/milvus/client/v2/entity"
	"github.com/milvus-io/milvus/client/v2/milvusclient"
	"log"
	"net/http"
	"os"
)

type CreateCollectionRequest struct {
	CollectionName string `json:"collectionName"`
}

type DeleteCollectionRequest struct {
	CollectionName string `json:"collectionName"`
}

type GetCollectionsResponse struct {
	CollectionNames []string `json:"collectionNames"`
}

func main() {

	dbUrl, exists := os.LookupEnv("DB_URL")

	// If environment variable is not set, set to default value
	if !exists {
		log.Println("DB_URL environment variable not set")
		dbUrl = "http://192.168.55.1:19530"
	}

	c, err := milvusclient.New(context.TODO(), &milvusclient.ClientConfig{
		Address: dbUrl,
	})

	if err != nil {
		panic(err)
	}

	defer func(c *milvusclient.Client, ctx context.Context) {
		err := c.Close(ctx)
		if err != nil {
			panic(err)
		}
	}(c, context.TODO())

	r := chi.NewRouter()
	r.Use(middleware.Logger)

	r.Get("/health", func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	})

	setUpDBRoutes(context.TODO(), r, c)

	err = http.ListenAndServe(":8002", r)

	if err != nil {
		return
	}
}

func setUpDBRoutes(context context.Context, r chi.Router, c *milvusclient.Client) {

	r.Get("/getCollections", func(w http.ResponseWriter, r *http.Request) {
		collectionNames, err := c.ListCollections(context, milvusclient.NewListCollectionOption())
		if err != nil {
			log.Printf("Error listing collections: %v", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		collections := GetCollectionsResponse{
			CollectionNames: collectionNames,
		}

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)

		if err := json.NewEncoder(w).Encode(collections); err != nil {
			log.Printf("Error encoding collections: %v", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

	})

	r.Options("/createCollection", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Headers", "*")
	})

	r.Post("/createCollection", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Headers", "*")

		var req CreateCollectionRequest
		err := json.NewDecoder(r.Body).Decode(&req)

		if err != nil {
			log.Printf("Decoding the request bodt failed: %v", err)
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}

		err = c.CreateCollection(context, milvusclient.NewCreateCollectionOption(
			req.CollectionName,
			createSchema(),
		))

		if err != nil {
			log.Printf("Failed to create collection: %v", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
	})

	r.Options("/removeCollection", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Headers", "*")
	})

	r.Delete("/removeCollection", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Headers", "*")

		var req DeleteCollectionRequest
		err := json.NewDecoder(r.Body).Decode(&req)

		if err != nil {
			log.Printf("Decoding the request bodt failed: %v", err)
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}

		err = c.DropCollection(context, milvusclient.NewDropCollectionOption(req.CollectionName))

		if err != nil {
			log.Printf("Failed to drop collection: %v", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
	})

}

func createSchema() *entity.Schema {
	return entity.NewSchema().
		WithField(entity.NewField().
			WithName("id").
			WithIsAutoID(true).
			WithDataType(entity.FieldTypeInt64).
			WithIsPrimaryKey(true)).
		WithField(entity.NewField().
			WithName("lectureUrl").
			WithDataType(entity.FieldTypeVarChar).
			WithMaxLength(512)).
		WithField(entity.NewField().
			WithName("vector").
			WithDataType(entity.FieldTypeFloatVector).
			WithDim(768)).
		WithField(entity.NewField().
			WithName("text").
			WithDataType(entity.FieldTypeVarChar).
			WithMaxLength(1024)).
		WithField(entity.NewField().
			WithName("timestamp_start").
			WithDataType(entity.FieldTypeFloat)).
		WithField(entity.NewField().
			WithName("timestamp_end").
			WithDataType(entity.FieldTypeFloat))
}
