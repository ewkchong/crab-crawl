# Goals
The main goal is to establish a web service that will efficiently crawl the web and index e-commerce products to be retrieved later. The service will eventually be run on a production system; as a component on the backend of a gift-sharing social media platform.

## Key Features
- **RESTful API**: An API will be exposed to the end application to perform various tasks (product page parsing, product retrieval)
- **Search Engine/Crawler**: The service will employ many workers to crawl the web for e-commerce products, capture key information, and index them for future retrieval
- **Load Balanced**: A load balancer will be utilized to increase availability and scalability for the end application

## Key Aspects
- **Performant**: The service will be optimized for low latency for the end user, so they may retrieve their desired results quickly
- **Fault Tolerant**: The service will be deployed on Kubernetes with auto-replaced dead nodes, automated rollbacks, and health-checked containers
- **Scalable**: An arbitrary number of instances of this service can be employed to enhance performance under high load

# Design

# Quick Start
To begin, you can run a single instance of the web service with the following command:
```bash
cargo run --bin crab-crawl
```
This will start the server on localhost:3000.

