-- Add up migration script here
CREATE TABLE IF NOT EXISTS fichier (
    id VARCHAR PRIMARY KEY,
    document_id VARCHAR (255) REFERENCES document,
    file_name VARCHAR (255),
    file_size BIGINT,
    mime_type VARCHAR (255),
    data_pointer VARCHAR (255),
    updated_by VARCHAR (255) NOT NULL,
    updated_on TIMESTAMP NOT NULL DEFAULT NOW(),
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);