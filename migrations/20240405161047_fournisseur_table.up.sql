-- Add up migration script here
CREATE TABLE IF NOT EXISTS fournisseur (
    id VARCHAR (255) PRIMARY KEY,
    code VARCHAR (255) NOT NULL,
    sigle VARCHAR (255) NOT NULL,
    designation VARCHAR (255) NOT NULL,
    telephone VARCHAR (255) NOT NULL,
    email VARCHAR (255) NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by VARCHAR (255) NOT NULL
); 