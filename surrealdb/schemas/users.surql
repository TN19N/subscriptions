# --- TABLE ---
DEFINE TABLE OVERWRITE users SCHEMAFULL
COMMENT 'Users table';

# --- FIELDS ---
DEFINE FIELD OVERWRITE username ON users TYPE string;
DEFINE FIELD OVERWRITE password ON users TYPE string;
DEFINE FIELD OVERWRITE created_at ON TABLE users TYPE datetime VALUE time::now() READONLY;

# --- INDEXES ---
DEFINE INDEX OVERWRITE username ON users COLUMNS username UNIQUE;
