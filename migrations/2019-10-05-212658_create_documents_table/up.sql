CREATE TABLE documents(
  id SERIAL,
  claim_id int4 REFERENCES claims(id) ON DELETE CASCADE,
  url TEXT,
  file_name TEXT,
  description TEXT,

  PRIMARY KEY (id,claim_id),

  created_at BIGINT,
  updated_at BIGINT,
  unique(id)
);

CREATE INDEX id_claim_id_index_documents ON documents (id, claim_id);

