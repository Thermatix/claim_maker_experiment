CREATE type loss_damage_type AS ENUM(
  'fire_damage',
  'water_damage',
  'mould_damage',
  'storm_damage',
  'theft_and_vandalism',
  'sinkhole',
  'marine__vessel',
  'business_interuption'
);

-- Too generic a type, could be used else where so DON'T DROP
CREATE type yn_type AS ENUM('yes', 'no');

CREATE TABLE claims(
  id SERIAL,
  user_id int4 REFERENCES users(id) ON DELETE CASCADE,
  against loss_damage_type,
  date_of_incident BIGINT,
  structure_affected yn_type,

  PRIMARY KEY (id, user_id),

  created_at BIGINT,
  updated_at BIGINT,
  unique(id)
);

CREATE INDEX id_user_id_index_claims ON claims (id, user_id);

