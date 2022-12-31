CREATE TABLE mail (
  message_id TEXT PRIMARY KEY,
  recipients TEXT ARRAY,
  sent_by TEXT

  --body TEXT NOT NULL,
  --published BOOLEAN NOT NULL DEFAULT FALSE
)