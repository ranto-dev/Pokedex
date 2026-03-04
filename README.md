# Pokedex 🦀


<div>
  <p align="center">
    Pokedex REST API is a backend service built in Rust using Actix Web.  
    It exposes a RESTful interface to manage Pokémon data and stores information in MongoDB.
  </p>
  <br />
  <p>
    This project is a refactored version of a previous CLI-based Pokédex application, redesigned into a scalable and production-ready web API architecture.
  </p>
</div>


## Documentation

Base URL:

```
[http://localhost:8080](http://localhost:8080)
```

#### 🔹 Get All Pokemons

```
GET /pokemons
````

Response:

```json
[
  {
    "_id": "65f4b1b2c5f8d9e4c1234567",
    "nom": "Bulbasaur",
    "types": ["Grass", "Poison"],
    "total": 318,
    "hp": 45,
    "att": 49,
    "def": 49,
    "vitesse": 45,
    "id_evolution": 2
  }
]
````


#### 🔹 Get Pokemon By ID

```
GET /pokemons/{id}
```

Example:

```
GET /pokemons/65f4b1b2c5f8d9e4c1234567
```

Responses:

* `200 OK` → Pokemon found
* `404 Not Found`
* `400 Bad Request` → Invalid ObjectId format


#### 🔹 Create a Pokemon

```
POST /pokemons
Content-Type: application/json
```

Request body:

```json
{
  "nom": "Charmander",
  "types": ["Fire"],
  "total": 309,
  "hp": 39,
  "att": 52,
  "def": 43,
  "vitesse": 65,
  "id_evolution": 5
}
```

Response:

* `201 Created` → Returns inserted ObjectId


#### 🔹 Delete a Pokemon

```

DELETE /pokemons/{id}
```

Response:

* `200 OK`
* `404 Not Found`


## Running with Docker

Make sure Docker is installed.

```bash
# verify docker version
docker --version

# verify docker compose version
docker compose version
```

Start the project:

```bash
# build container with docker compose
docker compose up --build
```

The API will run on:

```
http://localhost:8080
```

MongoDB runs inside a container and is connected internally via environment variables.

## Tech Stack

* Rust
* Actix Web
* MongoDB
* Docker
* Docker Compose
* Serde (JSON serialization)
* MongoDB Rust Driver


## Upcoming Features

* Update Pokemon endpoint (PUT / PATCH)
* Pagination support
* Search by type or name
* Filtering and sorting
* Request validation
* Centralized error handling
* Logging middleware
* JWT authentication
* Swagger / OpenAPI documentation
* Unit and integration testing
* CI/CD pipeline
* Production deployment configuration


## Project Goals

* Practice building production-grade APIs in Rust
* Learn MongoDB integration
* Apply clean architecture principles
* Improve Docker-based development workflow
