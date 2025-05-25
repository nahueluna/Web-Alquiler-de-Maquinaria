DROP SCHEMA public CASCADE;
CREATE SCHEMA public;

CREATE TYPE machine_status AS ENUM ('available', 'rented', 'maintenance');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email varchar(254) UNIQUE NOT NULL,
    name varchar(100) NOT NULL,
    surname varchar(100) NOT NULL,
    psw_hash varchar(64) UNIQUE NOT NULL,
    salt varchar(16) NOT NULL,
    role smallint NOT NULL
);

CREATE TABLE user_info (
    id INTEGER PRIMARY KEY REFERENCES users(id),
    birthdate date NOT NULL,
    id_card varchar(30) UNIQUE NOT NULL,
    phone varchar(50) NULL
);

CREATE TABLE codes_2fa (
    id INTEGER PRIMARY KEY REFERENCES users(id),
    code INTEGER NOT NULL
);

CREATE TABLE machinery_models (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    brand TEXT NOT NULL,
    model TEXT NOT NULL,
    year INTEGER NOT NULL CHECK (year >= 1900),
    policy TEXT NOT NULL,
    description TEXT NOT NULL,
    price REAL NOT NULL
);

CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    street TEXT,
    number TEXT,
    city TEXT
);


CREATE TABLE machinery_units (
    id SERIAL PRIMARY KEY,
    serial_number TEXT UNIQUE NOT NULL,
    status machine_status NOT NULL,
    notes TEXT,
    assigned_at TIMESTAMP NOT NULL DEFAULT NOW(),
    model_id INTEGER NOT NULL REFERENCES machinery_models(id),
    location_id INTEGER NOT NULL REFERENCES locations(id)
);

CREATE TABLE machinery_location_history (
    unit_id INTEGER NOT NULL REFERENCES machinery_units(id),
    location_id INTEGER NOT NULL REFERENCES locations(id),
    assigned_at TIMESTAMP NOT NULL,
    unassigned_at TIMESTAMP NOT NULL,
    PRIMARY KEY (unit_id, location_id, assigned_at)
);
