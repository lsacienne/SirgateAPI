# ShooterAPIREST

## Setup  project

Everything is setup in the docker compose except for some environment variables that should be defined in the file `rust-api/.env` :

```bash
DATABASE_URL="postgresql://shooteradmin:ff@postgres:5432/ShooterDB" # Defined in the docker-compose
REDIS_URL="redis://redis:6379" # Defined in the docker compose
JWT_SECRET="???" # Name it the way you like
```

## Endpoint list

### GET Endpoints

| url | method | description | arguments |
| --- | ------ | ------ | --------------- |
| htpp://\<api-host\>/singleuser | GET | Get user informations | JWT token |
| htpp://\<api-host\>/client/finddgs | GET | Get a DGS using the matchmaking system | JWT token |
| htpp://\<api-host\>/friends/getall | GET | Get all user friends | JWT token |
| htpp://\<api-host\>/dgs/players | GET | Get a list of all the clients in the DGS | JWT token (server) |
| htpp://\<api-host\>/achievement/getall | GET | Get a list of all the achievements of the player | JWT token |

### POST Endpoints

| url | method | description | arguments | warning |
| --- | ------ | ----------- | --------- | ------- |
| htpp://\<api-host\>/register | POST | Register a player | username & email & password | - |
| htpp://\<api-host\>/login | POST | Login a player | (username & password) or (email & password) | - |
| htpp://\<api-host\>/logout | POST | Uncache a player | JWT token | Should use DELETE method |
| htpp://\<api-host\>/friends/add | POST | Add an user to the friends list | JWT token and friend's username | - |
| htpp://\<api-host\>/dgs/register | POST | Put DGS in cache | JWT token and DGS port | - |
| htpp://\<api-host\>/dgs/login | POST | Register or login a DGS a startup (should be called from docker) | username & email & password | - |
| htpp://\<api-host\>/dgs/addplayer | POST | Add an user to the DGS | JWT token (server) and player username | - |
| htpp://\<api-host\>/dgs/removeplayer | POST | Add an user to the friends list | JWT token and friend's username | Should be DELETE method |
| htpp://\<api-host\>/achievement/add | POST | Add achievement to the specified player | JWT token (server) and (client username & achievement name) | - |

### PUT Endpoints

| url | method | description | arguments |
| --- | ------ | ------ | --------------- |
| htpp://\<api-host\\>/rank/update | PUT | Modify player rank | JWT token and (username & rank name) |

## Bonus features

### Friends

| Friends list has been implemented.

- One can add a friend
- One can view friends list
- Friends list is refreshed in the main menu
- Friends list display the status of each friend

### Player Caching

| Player caching has been implemented.

- Players are cached when they login to the game
- They are uncached when they logout

## Docker missing part

Matchmaking does not work properly on its own by only starting the docker compsoe. By using `dgs/finddgs` endpoint, the user will get the address of the server but it will not automatically connect.

We try to handle this issue by creating the following docker compose services (with the corresponding Dockerfile) but we couldn't test it correctly due to disk space issues. 

```yaml
shooter-serv1:
build:
    context: shooter-server
    args:
    USERNAME: server1
    EMAIL: server1
    PASSWORD: a
ports:
    - "7777:7777"
depends_on:
    - webapp

networks:
    - server-network
shooter-serv2:
build:
    context: shooter-server
    args:
    USERNAME: server2
    EMAIL: server2
    PASSWORD: a
ports:
    - "7778:7777"
depends_on:
    - webapp

networks:
    - server-network
```

