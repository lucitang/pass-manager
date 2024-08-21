CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       email TEXT NOT NULL UNIQUE,
                       key TEXT NOT NULL,
                       vault TEXT NOT NULL
)