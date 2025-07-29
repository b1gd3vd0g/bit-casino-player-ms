# 🃏 BitCasino Player Microservice

This is the **Player Microservice** for the BitCasino platform, responsible for managing user registration, authentication, and player profile data. Built with **Rust**, **Axum**, and **SQLx**, it is designed to be lightweight, secure, and easily deployable.

---

## // TODO

The project is currently at a stable point, where all routes defined in the [documentation](/openapi.yaml) work as expected. The application can be successfully containerized using the Dockerfile.

The following items still need to be completed:

- Fully document any modules which are not sufficiently commented.
- Add unit and integration tests
- Deploy to production (this should wait until more services are made ready)

---

## 🚀 Features

- 🔐 Secure user registration & login
- 🧂 Password hashing using Argon2
- 📆 Timestamps for user creation and updates
- 📄 JWT-based authentication (Bearer tokens)
- 🗃️ PostgreSQL integration via SQLx
- 🧪 Unit & integration tests
- ⚙️ Environment-based configuration
- 🐳 Docker-ready for containerized deployment

---

## 🧱 Tech Stack

- **Rust** – Safe and fast systems programming
- **Axum** – Web framework built on hyper
- **SQLx** – Async PostgreSQL driver with compile-time query checks
- **Tokio** – Async runtime
- **dotenv** – Load configuration from `.env`
- **uuid / chrono** – For UUIDs and timestamps
- **Docker** – Containerization
- **Terraform** *(optional)* – Infrastructure-as-code

---

## How to use this repository

### Branches

- **master** - Stable, production-ready code.
  - As of right now, this branch should only include code that has been tested thoroughly on the `compose` branch.
- **compose** - Used to test how this service integrates with the other bit casino services using docker compose.
  - This branch will utilize its own private postgres database container.
  - This branch no longer uses the dotenv crate to load environment variables.
  - The docker-compose.yaml file is defined externally to this service.
- **docker** - Used to test that this service can be containerized properly.
  - This branch will utilize a postgres database shared with the other services that is hosted on my local machine.
  - This branch uses the dotenv crate to load environment variables.
  - This branch can be containerized and tested individually, or alongside any of the other services.
- **local** - Used to test that this service can be built and run on my local machine.
  - This branch will utilize a postgres database shared with the other services on my local machine.
  - This branch uses the dotenv crate to load environment variables.
  - This branch should is intended to be run simply by using "cargo run".

