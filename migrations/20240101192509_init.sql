CREATE TABLE "user" (
    id serial NOT NULL PRIMARY KEY,
    uuid uuid NOT NULL,
    user_name varchar(64) NOT NULL,
    password_hash varchar(256),
    UNIQUE (uuid),
    UNIQUE (user_name)
);

CREATE TABLE "group" (
    id serial NOT NULL PRIMARY KEY,
    uuid uuid NOT NULL,
    group_name varchar(64) NOT NULL,
    root boolean default(false) NOT NULL,
    UNIQUE (uuid),
    UNIQUE (group_name)
);

CREATE TABLE "user_groups" (
    u_id serial NOT NULL REFERENCES "user" (id),
    g_id serial NOT NULL REFERENCES "group" (id),
    PRIMARY KEY (u_id, g_id)
);

CREATE TABLE "link_tree" (
    id serial NOT NULL PRIMARY KEY,
    uuid uuid NOT NULL,
    name varchar(64) NOT NULL,
    short_url varchar(20) NOT NULL,
    named_url varchar(64) default(NULL),
    root boolean default(false) NOT NULL,
    tree boolean default(false) NOT NULL,
    redir_link TEXT default(NULL),
    g_id serial NOT NULL REFERENCES "group" (id),
    UNIQUE (uuid),
    UNIQUE (name),
    UNIQUE (short_url),
    UNIQUE (named_url)
);

CREATE TABLE "link_entry" (
    id serial NOT NULL PRIMARY KEY,
    uuid uuid NOT NULL,
    name varchar(64) NOT NULL,
    redir_link TEXT default(NULL),
    g_id serial NOT NULL REFERENCES "group" (id),
    UNIQUE (uuid),
    UNIQUE (name)
);

CREATE TABLE "link_tree_entry" (
    t_id serial NOT NULL REFERENCES "link_tree" (id),
    e_id serial NOT NULL REFERENCES "link_entry" (id),
    PRIMARY KEY (t_id, e_id)
);

CREATE TABLE "entry_acl" (
    e_id serial NOT NULL REFERENCES "link_entry" (id),
    g_id serial NOT NULL REFERENCES "group" (id),
    read bool default(false) NOT NULL,
    write bool default(false) NOT NULL,
    PRIMARY KEY (e_id, g_id)
);

CREATE TABLE "tree_acl" (
    t_id serial NOT NULL REFERENCES "link_tree" (id),
    g_id serial NOT NULL REFERENCES "group" (id),
    read bool default(false) NOT NULL,
    write bool default(false) NOT NULL,
    PRIMARY KEY (t_id, g_id)
);