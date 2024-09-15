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
CREATE TABLE users
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT NOT NULL UNIQUE,
    email      TEXT NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME DEFAULT NULL
);

CREATE INDEX users_name_index ON users (name);
CREATE INDEX users_email_index ON users (email);
CREATE INDEX users_deleted_at_index ON users (deleted_at);

CREATE TABLE statuses
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    description TEXT
);

CREATE TABLE task_types
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    description TEXT
);

CREATE TABLE tasks
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id      INTEGER NOT NULL,
    created_by   INTEGER NOT NULL,
    status_id    INTEGER NOT NULL,
    parent_id    INTEGER,
    task_type_id INTEGER NOT NULL,
    title        TEXT    NOT NULL,
    description  TEXT,
    created_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at   DATETIME DEFAULT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (created_by) REFERENCES users (id),
    FOREIGN KEY (status_id) REFERENCES statuses (id),
    FOREIGN KEY (parent_id) REFERENCES tasks (id),
    FOREIGN KEY (task_type_id) REFERENCES task_types (id)
);

CREATE INDEX tasks_user_id_index ON tasks (user_id);
CREATE INDEX tasks_created_by_index ON tasks (created_by);
CREATE INDEX tasks_status_id_index ON tasks (status_id);
CREATE INDEX tasks_parent_id_index ON tasks (parent_id);
CREATE INDEX tasks_task_type_id_index ON tasks (task_type_id);
CREATE INDEX tasks_deleted_at_index ON tasks (deleted_at);

CREATE TABLE comments
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id    INTEGER NOT NULL,
    user_id    INTEGER NOT NULL,
    comment    TEXT    NOT NULL CHECK (LENGTH(comment) > 0),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME DEFAULT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX comments_task_id_index ON comments (task_id);
CREATE INDEX comments_user_id_index ON comments (user_id);
CREATE INDEX comments_deleted_at_index ON comments (deleted_at);

-- triggers to update updated_at column
CREATE TRIGGER update_users_updated_at
    AFTER UPDATE
    ON users
BEGIN
    UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_tasks_updated_at
    AFTER UPDATE
    ON tasks
BEGIN
    UPDATE tasks SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_comments_updated_at
    AFTER UPDATE
    ON comments
BEGIN
    UPDATE comments SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
```
