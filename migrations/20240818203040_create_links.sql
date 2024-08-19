-- Add migration script here

CREATE TABLE IF NOT EXISTS links
(
    id uuid DEFAULT gen_random_uuid(),
    url_path VARCHAR(24) NOT NULL,

    title TEXT NOT NULL
);

CREATE UNIQUE INDEX link_url_path
ON links (url_path)
