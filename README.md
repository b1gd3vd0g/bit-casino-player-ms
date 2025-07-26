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