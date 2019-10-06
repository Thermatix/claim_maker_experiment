CREATE TYPE p_number_type AS ENUM('home', 'mobile', 'work', 'other');

CREATE TABLE phone_numbers (
  id SERIAL,
  identity_id int4 REFERENCES identity(id) ON DELETE CASCADE,
  is_a p_number_type,
  name VARCHAR(50),
  number INTEGER,
  extention SMALLINT,
  calling_code SMALLINT,

  PRIMARY KEY (id, identity_id),

  created_at BIGINT,
  updated_at BIGINT,
  unique(id)
);

CREATE INDEX id_identity_id_index_phone_numbers ON phone_numbers (id, identity_id);

