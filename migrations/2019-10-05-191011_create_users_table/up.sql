CREATE TABLE users(
  id SERIAL PRIMARY KEY,
  created_at BIGINT,
  updated_at BIGINT,
  unique(id)
);

CREATE INDEX id_index_users ON users (id);

