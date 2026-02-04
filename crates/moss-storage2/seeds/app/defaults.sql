BEGIN;

INSERT OR IGNORE INTO kv(key, value) VALUES
  ('workbench.activityBarItemState.viewGroup.projects', CAST('{"order":1}' AS BLOB)),
  ('workbench.activityBarItemState.viewGroup.environments', CAST('{"order":2}' AS BLOB)),
  ('workbench.activityBarItemState.viewGroup.sourceControl', CAST('{"order":3}' AS BLOB));

COMMIT;