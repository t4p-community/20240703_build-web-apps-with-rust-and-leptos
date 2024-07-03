-- Add migration script here
CREATE TABLE color (
  id VARCHAR NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  hex_code VARCHAR NOT NULL
);

INSERT INTO color (id, name, hex_code) VALUES ('133191a0-d9f1-43fe-8b33-b0eff1439173', 'Red', 'FF0000');
INSERT INTO color (id, name, hex_code) VALUES ('e7651096-2609-45a5-b5a1-b3323f70d4f1', 'Green', '00FF00');
INSERT INTO color (id, name, hex_code) VALUES ('8d6f7327-da2f-45cc-9365-a720e06f4358', 'Blue', '0000FF');