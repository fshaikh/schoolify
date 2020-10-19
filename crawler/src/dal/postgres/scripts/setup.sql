/*
Setup script for database
*/
DROP TABLE schools;
DROP TABLE catchmentareas;
DROP TABLE regions;
DROP EXTENSION postgis;

CREATE EXTENSION postgis;

CREATE TABLE regions (
    id      UUID PRIMARY KEY,
    created_at timestamp NOT NULL,
    modified_at timestamp,
    created_by TEXT,
    version smallint ,
    key    TEXT NOT NULL,
    name    TEXT NOT NULL,
    country    TEXT NOT NULL,
    continent    TEXT NOT NULL
);

CREATE TABLE catchmentareas (
    id  UUID PRIMARY KEY,
    created_at timestamp NOT NULL,
    modified_at timestamp,
    created_by TEXT,
    version smallint ,
    area_key    TEXT NOT NULL,
    district_key    TEXT NOT NULL,
    district_name    TEXT NOT NULL,
    polygon GEOMETRY NOT NULL,
    region_id  UUID REFERENCES regions(id) NOT NULL
);

CREATE TABLE schools (
    Id  UUID PRIMARY KEY,
    created_at timestamp NOT NULL,
    modified_at timestamp,
    created_by TEXT,
    version smallint ,
    school_id    TEXT NOT NULL,
    school_name TEXT NOT NULL,
    school_type    smallint,
    primary_phone_number TEXT,
    email TEXT,
    url TEXT,
    contact_persons TEXT[],
    languages TEXT[],
    is_bilingual BOOLEAN,
    funding_type TEXT,
    address TEXT,
    district  TEXT ,
    fees TEXT,
    location GEOMETRY NOT NULL,
    region_id  UUID REFERENCES regions(id) NOT NULL,
    catchmentarea_id  UUID REFERENCES catchmentareas(id) NULL
);

CREATE INDEX schools_regionid_idx ON schools (region_id);
CREATE INDEX schools_catchmentareaid_idx ON schools (catchmentarea_id);

-- 