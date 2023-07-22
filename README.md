# Rust API Development With Axum and EdgeDB

## Setup

1. Make sure [Docker](https://www.docker.com/) is installed and running
2. The default port for the backend api is 8080. You can change this from `docker-compose.yml` if needed

## Run Locally using Docker
1. Run API via Docker Compose:
    ```bash
    docker-compose up
    ```
2. Test routes:
   * You can use e.g [Postman](https://www.postman.com/)
   * If you are using VSCode, you can use api-requests.http file.


## REST API

### Health check

#### GET /health_check

Checks that the app is ok.

### Users

#### GET /users

Example response payload:

```
[
  {
    "name": "Graydon Hoare",
    "email": "graydonhoare@mozilla.com
  }
]
```

#### POST /user

Example request payload:

```
{
    "name": "Graydon Hoare",
    "email": "graydonhoare@mozilla.com
}

```

#### DELETE /user

Example request payload:

```
{
    "name": "Graydon Hoare",
    "email": "graydonhoare@mozilla.com
}

```

## Used Tech

- [Rust (1.71.0)](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [EdgeDB](https://www.edgedb.com/)
- [Docker](https://www.docker.com/)
