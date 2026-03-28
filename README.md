# 🦀 Pokedex API

<div align="center">

<img src="https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/25.png" width="200" alt="Pikachu"/>

### ⚡ Pokedex REST API built with Rust & Actix Web

</div>

<br/>

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white) ![Actix](https://img.shields.io/badge/Actix_Web-3C3C3C) ![MongoDB](https://img.shields.io/badge/MongoDB-4EA94B?logo=mongodb&logoColor=white) ![Docker](https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=white) ![License](https://img.shields.io/badge/License-MIT-yellow.svg) ![Status](https://img.shields.io/badge/Status-In_Progress-orange)

## 📖 Description

Pokedex REST API is a backend service built in **Rust** using **Actix Web**.
It exposes a RESTful interface to manage Pokémon data and stores information in **MongoDB**.

This project is a refactored version of a previous CLI-based Pokédex application, redesigned into a scalable and production-ready web API architecture.

## 🌐 Documentation

You can read documentation from [Here](/docs/api.md)

## ✨ Features

- **RESTful API Design**
  Clean and structured endpoints following REST principles for managing Pokémon data.

- **CRUD Operations**
  Create, retrieve, and delete Pokémon records with ease.

- **High Performance with Rust**
  Built using Rust and Actix Web for fast and memory-safe backend performance.

- **MongoDB Integration**
  NoSQL database for flexible and scalable data storage.

- **Dockerized Environment**
  Fully containerized setup using Docker and Docker Compose for easy deployment.

- **JSON Serialization with Serde**
  Efficient data parsing and serialization using Serde.

- **Modular Architecture**
  Clean and maintainable code structure following best backend practices.

- **Error Handling**
  Proper HTTP status codes and structured error responses.

- **API Ready for Scalability**
  Designed to be extended with authentication, pagination, and filtering.

## 🧰 Tech Stack

- 🦀 Rust
- ⚡ Actix Web
- 🍃 MongoDB
- 🐳 Docker
- 📦 Docker Compose
- 🔄 Serde (JSON serialization)
- 🔗 MongoDB Rust Driver

## 🚀 Upcoming Features

- [ ] Update Pokemon (PUT / PATCH)
- [ ] Pagination support
- [ ] Search by type or name
- [ ] Filtering and sorting
- [ ] Request validation
- [ ] Centralized error handling
- [ ] Logging middleware
- [ ] JWT authentication
- [ ] Swagger / OpenAPI docs
- [ ] Unit & integration tests
- [ ] CI/CD pipeline
- [ ] Production deployment

## 🎯 Project Goals

- Build production-grade APIs in Rust
- Learn MongoDB integration
- Apply clean architecture principles
- Improve Docker workflow
