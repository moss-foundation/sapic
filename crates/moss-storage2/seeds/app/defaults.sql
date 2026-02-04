BEGIN;

INSERT OR IGNORE INTO kv(key, value) VALUES
  ('workbench.activityBarItemState.placeholder.view.group.projects', CAST('{"order":1}' AS BLOB)),
  ('workbench.activityBarItemState.placeholder.view.group.environments', CAST('{"order":2}' AS BLOB)),
  ('workbench.activityBarItemState.placeholder.view.group.sourceControl', CAST('{"order":3}' AS BLOB));

COMMIT;