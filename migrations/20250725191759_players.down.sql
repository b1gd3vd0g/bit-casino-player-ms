-- Add down migration script here
DROP TABLE IF EXISTS players;

DROP EXTENSION IF EXISTS "pgcrypto";