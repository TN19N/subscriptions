# --- TABLE ---
DEFINE TABLE OVERWRITE subscriptions SCHEMAFULL
COMMENT 'Subscription table';

# --- FIELDS ---
DEFINE FIELD OVERWRITE email ON subscriptions TYPE string ASSERT string::is::email($value);
DEFINE FIELD OVERWRITE name ON subscriptions TYPE string;
DEFINE FIELD OVERWRITE created_at ON TABLE subscriptions TYPE datetime VALUE time::now() READONLY;

# --- INDEXES ---
DEFINE INDEX OVERWRITE unique_email ON subscriptions COLUMNS email UNIQUE;
