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
