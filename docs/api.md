# API Documentation

### Base URL

```
http://localhost:8080
```

#### 🔹 Get All Pokemons

```
GET /pokemons
```

##### Response

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
```

#### 🔹 Get Pokemon By ID

```
GET /pokemons/{id}
```

##### Example

```
GET /pokemons/65f4b1b2c5f8d9e4c1234567
```

##### Responses

- `200 OK` → Pokemon found
- `404 Not Found`
- `400 Bad Request` → Invalid ObjectId format

#### 🔹 Create a Pokemon

```
POST /pokemons
Content-Type: application/json
```

##### Request Body

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

##### Response

- `201 Created` → Returns inserted ObjectId

#### 🔹 Delete a Pokemon

```
DELETE /pokemons/{id}
```

##### Responses

- `200 OK`
- `404 Not Found`

### Running with Docker

Make sure Docker is installed.

```bash
# Verify docker version
docker --version

# Verify docker compose version
docker compose version
```

### Start the project

```bash
docker compose up --build
```

MongoDB runs inside a container and is connected internally via environment variables.

### Access API

```
http://localhost:8080
```
