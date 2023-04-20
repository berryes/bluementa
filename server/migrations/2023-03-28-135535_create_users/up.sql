CREATE TABLE users (
  id VARCHAR(36) ,
  premission_level TINYINT,
  username VARCHAR(255),
  password VARCHAR(255),
  display_name VARCHAR(255),
  PRIMARY KEY(id)
)