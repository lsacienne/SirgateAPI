# ShooterAPIREST

## Setup  project

Everything is setup in the docker compose except for some environment variables that should be defined in the file `rust-api/.env` :

```bash
DATABASE_URL="postgresql://shooteradmin:ff@postgres:5432/ShooterDB" # Defined in the docker-compose
REDIS_URL="redis://redis:6379" # Defined in the docker compose
JWT_SECRET="???" # Name it the way you like
```

## Endpoint list (postgresql)

### GET Endpoints

| url | method | description | arguments |
| --- | ------ | ------ | --------------- |
| - | GET | Get a user by username | - |
| - | GET | Get a user by email | - |
| - | GET | Get all users | - |
| - | GET | Get all friends of a player | - |
| - | 

### POST Endpoints

| url | method | description | arguments |
| --- | ------ | ------ | --------------- |
| - | POST | Register a player | username & email & password |
| - | POST | Login a player | username & password |
| - | POST | Login a player | email & password |
| - | POST | Create a friendship | uuid_client1 & uuid_client2 |
| - | POST | Grant achievement to a player | uuid_client & achievement_name |

### PUT Endpoints

| url | method | description | arguments |
| --- | ------ | ------ | --------------- |
| - | PUT | Modify player rank | uuid_client & rank_name |

## Endpoint list (redis)

### GET Endpoints

| url | method | description | arguments |
| --- | ------ | ------ | --------------- |
| - | GET | Get all available parties | - |
| - | GET | Get a party for the specified rank (prefer non-empty parties) | - |

### POST Endpoints

### PUT Endpoints