-- Add migration script here

INSERT INTO "user" VALUES (1, '00000000-0000-0000-0000-000000000000', 'admin', '$pbkdf2-sha256$i=600000,l=32$h6qBhZVxHvevD+5qv8bclQ$ACNRLq7BKuZ70AB6QqknR6pct3wTH2rhnFpZ6PGA6Bk');
INSERT INTO "group" VALUES (1, gen_random_uuid(), 'root', true);
INSERT INTO "user_groups" VALUES (1, 1);