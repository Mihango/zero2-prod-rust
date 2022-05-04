-- Add migration script here
-- Create subscription table
CREATE TABLE subscriptions (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);