DROP SCHEMA public CASCADE;
CREATE SCHEMA public;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email varchar(254) UNIQUE NOT NULL,
    name varchar(100) NOT NULL,
    surname varchar(100) NOT NULL,
    birthdate date NOT NULL,
    id_card varchar(30) UNIQUE NOT NULL,
    phone varchar(50) NULL,
    password varchar(100) NOT NULL,
    role smallint NOT NULL
);

CREATE TABLE machines (
    id SERIAL PRIMARY KEY,
    serial_number varchar(50) UNIQUE NOT NULL,
    name varchar(150) NOT NULL,
    brand varchar(50) NOT NULL,
    model varchar(50) NOT NULL,
    year smallint NOT NULL,
    policy varchar(300) NOT NULL,
    description varchar(500) NOT NULL,
    price real NOT NULL
);
