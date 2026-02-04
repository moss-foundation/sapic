BEGIN;

INSERT OR IGNORE INTO kv(key, value) VALUES
  ('1', CAST('A' AS BLOB)),
  ('2', CAST('B' AS BLOB)),
  ('3', CAST('C' AS BLOB));

COMMIT;