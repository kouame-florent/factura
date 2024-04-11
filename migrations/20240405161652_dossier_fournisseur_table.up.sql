-- Add up migration script here
CREATE TABLE IF NOT EXISTS dossier_fournisseur (
    id VARCHAR PRIMARY KEY,
    fournisseur_id VARCHAR (255) REFERENCES fournisseur,
    designation VARCHAR (255) NOT NULL,
    numero_courier VARCHAR (255) NOT NULL,
    date_creation TIMESTAMP NOT NULL,
    updated_by VARCHAR (255) NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);