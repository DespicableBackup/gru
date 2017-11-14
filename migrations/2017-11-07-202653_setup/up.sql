CREATE TABLE minions (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  active BOOLEAN NOT NULL DEFAULT false,
  key VARCHAR,
  ip VARCHAR,
  port INTEGER DEFAULT 22,
  username VARCHAR,
  directory VARCHAR
);
CREATE UNIQUE INDEX minions_unique ON minions(name);
