CREATE type property_type AS ENUM(
  'house',
  'apartment__flat',
  'commercial_premises'
);

CREATE TABLE addresses(
  id SERIAL,
  user_id int4 REFERENCES users(id) ON DELETE CASCADE,
  door_number INTEGER,
  postcode VARCHAR(10),
  description property_type,

  PRIMARY KEY (id, user_id),

  created_at BIGINT,
  updated_at BIGINT,
  unique(id)
);

CREATE INDEX id_claim_id_index_addresses ON addresses (id, user_id);

