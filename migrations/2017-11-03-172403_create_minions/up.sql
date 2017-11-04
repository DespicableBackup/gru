CREATE TABLE minions (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  active BOOLEAN NOT NULL DEFAULT false
);
CREATE UNIQUE INDEX minions_unique ON minions(name);
