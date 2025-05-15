CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email varchar(254) UNIQUE NOT NULL,
    name varchar(100) NOT NULL,
    password varchar(100) NOT NULL,
    role smallint NOT NULL
);
