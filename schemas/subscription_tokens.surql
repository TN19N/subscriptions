# --- TABLE ---
DEFINE TABLE OVERWRITE subscription_tokens SCHEMAFULL
COMMENT 'Subscription Tokens table';

# --- FIELDS ---
DEFINE FIELD OVERWRITE token ON subscription_tokens TYPE string;
DEFINE FIELD OVERWRITE created_at ON TABLE subscription_tokens TYPE datetime VALUE time::now() READONLY;

# --- INDEXES ---
DEFINE INDEX OVERWRITE unique_token ON subscription_tokens COLUMNS token UNIQUE;
