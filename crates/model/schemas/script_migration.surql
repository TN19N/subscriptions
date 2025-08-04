# --- TABLE ---
DEFINE TABLE OVERWRITE script_migration SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete NONE
    COMMENT 'Script migration table';

# --- FIELDS ---
DEFINE FIELD OVERWRITE script_name ON script_migration TYPE string;
DEFINE FIELD OVERWRITE executed_at ON script_migration TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE checksum ON script_migration TYPE option<string>;
