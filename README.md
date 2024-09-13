# Proof of Concept - Simple Task Manager API

## Table of Contents

1. [Project Overview](#project-overview)
2. [Technologies Used](#technologies-used)
3. [Setup Instructions](#setup-instructions)
4. [API Routes](#api-routes)
5. [Database Schema](#database-schema)
6. [License](#license)

## Project Overview

The Simple Task Manager API is a proof of concept project designed to demonstrate how to build a web-based RESTful API
using Rust with the Axum framework, SQLx library, and SQLite as the database. This API allows users to manage tasks and
users, showcasing basic CRUD (Create, Read, Update, Delete) operations. This project serves as an example for building a
scalable and maintainable backend service, highlighting the modern development practices in Rust.

### Features

- **Task Management**: Create, read, update, and delete tasks.
- **User Management**: Create, read, update, and delete users.
- **Relational Data**: Associate tasks with specific users.
- **Status Tracking**: Track the status of tasks with predefined states (created, in-progress, finished, broken,
  accepted).

### Why Axum and SQLx

- **Axum**: A web framework built on Tokio, designed for ergonomics and modularity, making it suitable for building
  robust web applications.
- **SQLx**: An asynchronous SQL toolkit that provides compile-time checked queries, making database interactions safe
  and efficient.
- **SQLite**: A lightweight and self-contained database engine that is perfect for prototyping and small applications.

## Technologies Used

1. **Rust**: A systems programming language known for its performance, safety, and concurrency capabilities.
2. **Axum**: A highly ergonomic and modular web application framework for Rust.
3. **SQLx**: An asynchronous, compile-time checked SQL toolkit with support for various databases, including SQLite.
4. **SQLite**: A compact, efficient, and zero-configuration SQL database engine.

## Setup Instructions

1. **Run Database Migrations**:
   Ensure `sqlite3` CLI is installed and available in your PATH.
    ```sh
    sqlite3 db.sqlite < migrations/schema.sql
    ```   

2. **Build and Run the API Server**:
    ```sh
    cargo run
    ```

The API server will start at `http://127.0.0.1:1337`.

## API Routes

### Tasks

| Endpoint      | Request Type | Description       |
|---------------|--------------|-------------------|
| `/tasks`      | `POST`       | Create a new task |
| `/tasks/{id}` | `GET`        | Get task by ID    |
| `/tasks`      | `GET`        | Get all tasks     |
| `/tasks/{id}` | `PUT`        | Update task by ID |
| `/tasks/{id}` | `DELETE`     | Delete task by ID |

### Users

| Endpoint      | Request Type | Description       |
|---------------|--------------|-------------------|
| `/users`      | `POST`       | Create a new user |
| `/users/{id}` | `GET`        | Get user by ID    |
| `/users`      | `GET`        | Get all users     |
| `/users/{id}` | `PUT`        | Update user by ID |
| `/users/{id}` | `DELETE`     | Delete user by ID |

## Database Schema

The database schema includes two tables: `tasks` and `users`. Here’s the structure:

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME DEFAULT NULL
);

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    created_by INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status CHAR(1) NOT NULL DEFAULT 'c', -- Created (c), Accepted (a), In Progress (p), Finished (f), Broken (b)
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME DEFAULT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (created_by) REFERENCES users (id)
);
```
