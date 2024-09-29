CREATE TABLE tokens (
    id BIGSERIAL PRIMARY KEY,
    external_key VARCHAR(100) NOT NULL,
    generated_key UUID UNIQUE NOT NULL,
    created TIMESTAMP WITHOUT TIME ZONE,
    applied TIMESTAMP WITHOUT TIME ZONE,
    ttl INTEGER,
);