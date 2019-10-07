CREATE TYPE claiment_type AS ENUM(
  'homeowner',
  'business',
  'landlord__property_manager'
);

CREATE TABLE identity(
  id SERIAL,
  user_id int4 REFERENCES users(id) ON DELETE CASCADE,
  is_a claiment_type,
  first_name VARCHAR(50),
  last_name VARCHAR(256),
  email VARCHAR(256),

  PRIMARY KEY (id, user_id),

  created_at BIGINT,
  updated_at BIGINT,
  unique(id)
);

CREATE INDEX id_user_id_index_identity ON identity (id, user_id);

