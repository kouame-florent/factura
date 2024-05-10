-- Add up migration script here
CREATE TABLE IF NOT EXISTS document (
    id VARCHAR PRIMARY KEY,
    code VARCHAR (255),
    libelle VARCHAR (255),
    categorie VARCHAR (255),
    date_signature TIMESTAMP,
    signataire VARCHAR (255),
    montant BIGINT,
    dossier_fournisseur_id VARCHAR (255) REFERENCES dossier_fournisseur,
    updated_by VARCHAR (255) NOT NULL,
    updated_on TIMESTAMP NOT NULL DEFAULT NOW(),
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
