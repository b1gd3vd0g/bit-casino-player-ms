# Bit Casino -- Player Microservice

> [!NOTE]
> This service is currently **stable** but under development.

A **REST API** written in **Rust** handling **player accounts** and **authentication** for **Bit Casino** - a virtual gambling simulator.

### Features

- Secure password storage, hashing using Argon2.
- Safe, easy authentication using JSON Web Tokens and Bearer Authentication.

## How to use this repository

This service is not very useful on its own. It relies upon the [**Currency Microservice**](https://github.com/b1gd3vd0g/bit-casino-currency-ms) and a **PostgreSQL** database.

To test this API alongside the whole environment, you can follow the instructions in the [Infrastructure](https://github.com/b1gd3vd0g/bit-casino-infra) repository to test all services locally using **Docker Compose**.

You can then interact via the frontend at `localhost:60000` or call the integrated player microservice directly at `localhost:60600`.

## Functionality

The player microservice currently supports the following functions:

- Create a new player account
- Delete a player account
- Authenticate a player's login credentials
- Authenticate a player via JWT (provided by creation/login functions)

## Related Repositories

- [Currency Microservice](https://github.com/b1gd3vd0g/bit-casino-currency-ms) - Handles bit wallet creation and safe transactions.
- [Reward Microservice](https://github.com/b1gd3vd0g/bit-casino-reward-ms) - Handles daily bonus claims and streaks.
- [Slots Microservice](https://github.com/b1gd3vd0g/bit-casino-slots-ms) - Handles the backend for the custom slot machine game **Byte Builder**.
- [Frontend](https://github.com/b1gd3vd0g/bit-casino-frontend) - A react app creating a user-friendly interface with which to interact with the backend.
- [Infrastructure](https://github.com/b1gd3vd0g/bit-casino-infra) - Allows for integration testing locally using **docker compose**.
