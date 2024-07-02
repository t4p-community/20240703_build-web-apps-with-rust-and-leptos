-- Add migration script here
CREATE TABLE car (
    id VARCHAR NOT NULL PRIMARY KEY,
    make VARCHAR NOT NULL,
    model VARCHAR NOT NULL,
    year INTEGER NOT NULL,
    color VARCHAR NOT NULL,
    price REAL NOT NULL
);

INSERT INTO car (id, make, model, year, color, price)
VALUES ('133191a0-d9f1-43fe-8b33-b0eff1439173', 'Ford', 'Fusion', 2018, 'Red', 20000.00);
INSERT INTO car (id, make, model, year, color, price) 
VALUES ('e7651096-2609-45a5-b5a1-b3323f70d4f1', 'Tesla', 'Model S', 2020, 'Blue', 80000.00);
