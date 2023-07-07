<h1 align="center">Busquery API</h1>

<p align="center">
    <img src="../wiki/busquery.svg" width="500">
</p>

## About

This [REST API](https://restfulapi.net/) is based on
[Actix Web](https://actix.rs/), a [Rust](https://www.rust-lang.org/)
web library. It is to be used as the general backend for the Busquery
web application.   

## Setup

To compile the server, you must have a `.env` file in the root of the
executable, with the `DATABASE_URL` set to a MySQL connection with
the format `mysql://username:password@127.0.0.1/db`. This is because
SQLx needs this environment variable to do it's compile time check of
the SQL queries on the correct MySQL or MariaDB server. All the other
environment variables are related to authentication and security, so
they should never be committed under __any__ circumstances! So under
development the test credentials and secrets will be loaded from a
`.env.dev` file, then in production a `.env` file will be used.   

To start up the development server, you run this docker command
```shell
docker compose -f compose-dev-server.yaml up
```

This starts up the SQL server with a
[phpMyAdmin](https://www.phpmyadmin.net/) web interface that can be
found at [http://localhost:6060](http://localhost:6060). Just import the
`setup.sql` to set up the SQL tables.   

## Routes

The API uses GET, POST, PUT and DELETE routes. The API utilizes URL
parameters and `application/json` bodies.   

---
### All businesses
`GET /business/all`   
Returns all the businesses in the database
<br>

This is an example query
```sh
curl "localhost:8080/business/all" | jq '.'
```

This query will give the output
```json
[
  {
    "org": 995412020,
    "notes": "Gjør kule IT greier"
  },
  {
    "org": 928372340,
    "notes": "Lokalavisen"
  }
]
```

---
### Add businesses
`POST /business/{org}/add`   
Adds a new business to the database
<br>

Some parameters are `url routed`.
| Parameter | Value |
| - | - |
| org | u32 |

Some parameters are `json`.
| Parameter | Value |
| - | - |
| notes | String |

This is an example query
```sh
curl -X POST "localhost:8080/business/928372340/add" \
   -H "Content-Type: application/json" \
   -d '{ "notes": "Lokalavisen ✨" }'
```

This query will give the output
```
Business successfully added
```

---
### Update businesss
`PUT /business/{org}/update`   
Updates the notes of a business in the database
<br>

Some parameters are `url routed`.
| Parameter | Value |
| - | - |
| org | u32 |

Some parameters are `json`.
| Parameter | Value |
| - | - |
| notes | String |

This is an example query
```sh
curl -X PUT "localhost:8080/business/928372340/update" \
   -H "Content-Type: application/json" \
   -d '{ "notes": "Lokalavisen" }'
```

This query will give the output
```
Business updated
```

---
### Remove businesses
`DELETE /business/{org}/update`   
Removes a business from the database
<br>

The parameters are `url routed`.
| Parameter | Value |
| - | - |
| org | u32 |

This is an example query
```sh
curl -X DELETE "localhost:8080/business/928372340/update"
```

This query will give the output
```
Business removed
```