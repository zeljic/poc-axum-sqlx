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