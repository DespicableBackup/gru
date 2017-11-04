CREATE TABLE minions_tmp (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  active BOOLEAN NOT NULL DEFAULT false
);
INSERT INTO minions_tmp SELECT id, name, active FROM minions;
DROP TABLE minions;
ALTER TABLE minions_tmp RENAME TO minions;
CREATE UNIQUE INDEX minions_unique ON minions(name);
