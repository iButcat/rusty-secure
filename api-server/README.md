# Rusty Secure API Server

## Overview

This is a simple API server built with Rust and Actix Web. It is designed to be used with the ESP32-CAM to capture images and send them to the server. The server will then process the image and return a response.

## Keep in mind

I love clean architecture and design patterns with modular layers like dtos, endpoints, services, repositories, models, etc... 

But for now, I'm just going to keep it simple or this project will take forever to complete.

## Run

```bash
cargo run
```

## TODO

- [ ] Add tests
- [ ] Add documentation
- [ ] Add authentication
- [ ] Add authorization
- [ ] Add health check
- [ ] Add swagger