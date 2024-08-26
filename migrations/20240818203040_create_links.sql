-- Add migration script here

CREATE TABLE IF NOT EXISTS links
(
    id uuid DEFAULT gen_random_uuid(),
    link_path VARCHAR(24) NOT NULL,

    title TEXT NOT NULL,
    description TEXT NOT NULL,
    image_url TEXT NOT NULL,

    android_destination TEXT NOT NULL,
    ios_destination TEXT NOT NULL,
    web_destination TEXT NOT NULL
);

CREATE UNIQUE INDEX link_path_index
ON links (link_path)
