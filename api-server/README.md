# Rusty Secure API Server

## Overview

This is a simple API server built with Rust and Actix Web. It is designed to be used with the ESP32-CAM to capture images and send them to the server. The server will then process the image and return a response.

## Architecture

This project adopts principles from **Hexagonal Architecture** (also known as Ports and Adapters) and **Clean Architecture**. Having extensively worked with Go microservices and client libraries, I've found this architectural pattern to be exceptionally effective for building maintainable, testable, and adaptable systems. This project represents my effort to bring these same benefits to the Rust ecosystem.

The core principle is to create clear boundaries between the business logic and external concerns like frameworks, databases, and third-party services. This is achieved through a structured layering of components and the use of interfaces (traits in Rust) to define contracts between layers.

### Project Structure

api-server/
├── Cargo.toml
├── src/
│ ├── main.rs # Entry point, DI setup, starts Actix
│ ├── app_state.rs # Defines the AppState struct holding service traits
│ ├── config/
│ │ ├── mod.rs # Exports configuration components
│ │ └── app_config.rs # Configuration loading
│ ├── domain/
│ │ ├── mod.rs # Exports domain models
│ │ ├── models/
│ │ │ ├── mod.rs
│ │ │ ├── status.rs # Status entity
│ │ │ └── picture.rs # Picture entity
│ │ └── errors/
│ │ ├── mod.rs
│ │ └── app_error.rs # Domain error types
│ ├── interfaces/
│ │ ├── mod.rs # Exports all ports
│ │ ├── repositories/
│ │ │ ├── mod.rs # Defines repository traits (output ports)
│ │ │ ├── status_repo.rs # StatusRepository trait
│ │ │ ├── picture_repo.rs # PictureRepository trait
│ │ │ └── storage_repo.rs # StorageRepository trait
│ │ └── services/
│ │ ├── mod.rs # Defines service traits (input ports)
│ │ ├── status_service.rs # StatusService trait
│ │ └── picture_service.rs # PictureService trait
│ ├── application/
│ │ ├── mod.rs # Exports service implementations
│ │ ├── services/
│ │ │ ├── mod.rs
│ │ │ ├── status_service.rs # StatusService implementation
│ │ │ └── picture_service.rs # PictureService implementation
│ │ └── dtos/
│ │ ├── mod.rs
│ │ ├── requests.rs # Request DTOs
│ │ └── responses.rs # Response DTOs
│ └── infrastructure/
│ ├── mod.rs # Exports infrastructure components
│ ├── adapters/
│ │ ├── mod.rs
│ │ ├── driving/
│ │ │ ├── mod.rs
│ │ │ ├── handlers/
│ │ │ │ ├── mod.rs
│ │ │ │ ├── status_handler.rs
│ │ │ │ └── picture_handler.rs
│ │ │ └── middleware/
│ │ │ ├── mod.rs
│ │ │ └── error_handler.rs
│ │ └── driven/
│ │ ├── mod.rs
│ │ ├── repositories/
│ │ │ ├── mod.rs
│ │ │ ├── mongo_repository.rs
│ │ │ └── gcs_repository.rs
│ │ └── clients/
│ │ ├── mod.rs
│ │ ├── mongo_client.rs
│ │ └── google_storage_client.rs
│ └── config/
│ ├── mod.rs
│ └── dependency_injection.rs
└── .env # Environment variables

### Key Architectural Components:

1. **Domain Layer**:
   - Contains the core business entities (`Status`, `Picture`)
   - Defines domain-specific errors
   - Independent of other layers, frameworks, or external dependencies

2. **Interfaces Layer**:
   - **Repository Traits (Output Ports)**: Define contracts for data access
   - **Service Traits (Input Ports)**: Define use cases and the capabilities of the application
   - Acts as the boundary of the hexagon, defining what goes in and out

3. **Application Layer**:
   - Implements the service traits defined in the interfaces layer
   - Contains the business logic and orchestrates calls to repositories
   - Depends on domain models and repository interfaces, but not on concrete implementations

4. **Infrastructure Layer**:
   - **Driving Adapters**: Handle inputs to the system (HTTP handlers, middlewares)
   - **Driven Adapters**: Implement repository interfaces using specific technologies (MongoDB, GCS)
   - Contains framework-specific code (Actix Web) and external service clients

5. **Dependency Injection**:
   - Wires everything together at startup
   - Service implementations (wrapped in `Arc<dyn Trait>`) are injected into the application state

### Benefits of This Approach:

- **Testability**: Each layer can be tested in isolation with mocks
- **Maintainability**: Clear boundaries make it easier to understand and modify the codebase
- **Adaptability**: Infrastructure details can be swapped without affecting business logic
- **Domain Focus**: Business rules are clearly represented in the domain layer

This approach has proven extremely effective in Go projects, and I'm happy to apply these same principles in Rust, leveraging its powerful type system and trait-based polymorphism.

## Run

```bash
cargo run
```

## TODO

- [ ] Add tests (unit tests for services/repos, integration tests for handlers)
- [ ] Add documentation (especially for public APIs/handlers)
- [ ] Add authentication
- [ ] Add authorization
- [ ] Add health check endpoint (`/health`)
- [ ] Add OpenAPI/Swagger documentation