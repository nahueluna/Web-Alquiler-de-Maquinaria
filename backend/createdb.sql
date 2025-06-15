DROP SCHEMA public CASCADE;
CREATE SCHEMA public;

CREATE TYPE user_status AS ENUM ('active', 'deleted');
CREATE TYPE machine_status AS ENUM ('available', 'rented', 'maintenance', 'reserved');
CREATE TYPE rental_status AS ENUM ('active', 'pending_payment', 'completed', 'cancelled', 'failed');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email varchar(254) UNIQUE NOT NULL,
    name varchar(100) NOT NULL,
    surname varchar(100) NOT NULL,
    psw_hash varchar(64) NOT NULL,
    salt varchar(16) NOT NULL,
    role smallint NOT NULL,
    refresh text NULL,
    status user_status NOT NULL
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

CREATE TABLE change_psw_codes (
    id INTEGER PRIMARY KEY REFERENCES users(id),
    code varchar(64) UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE machinery_models (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    brand TEXT NOT NULL,
    model TEXT NOT NULL,
    year INTEGER NOT NULL CHECK (year >= 1900),
    policy TEXT NOT NULL,
    description TEXT NOT NULL,
    price REAL NOT NULL,
    image varchar(64) NOT NULL
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

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE CHECK (name = LOWER(name))
);

CREATE TABLE machinery_categories (
    model_id INTEGER NOT NULL REFERENCES machinery_models(id),
    category_id INTEGER NOT NULL REFERENCES categories(id),
    PRIMARY KEY (model_id, category_id)
);

CREATE TABLE rentals (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    retirement_employee_id INTEGER NULL REFERENCES users(id),
    return_employee_id INTEGER NULL REFERENCES users(id),
    retirement_date DATE NULL,
    return_date DATE NULL,
    machine_id INTEGER NOT NULL REFERENCES machinery_units(id),
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    total_price REAL NOT NULL,
    status rental_status NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    payment_id TEXT NULL,
    notes TEXT NULL
);

CREATE TABLE model_extra_images (
    name varchar(64) PRIMARY KEY,
    id INTEGER NOT NULL REFERENCES machinery_models(id)
);

CREATE TABLE questions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    model_id INTEGER NOT NULL REFERENCES machinery_models(id),
    content varchar(256) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
