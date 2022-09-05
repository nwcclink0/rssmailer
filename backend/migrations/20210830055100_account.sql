-- Add migration script here
CREATE TABLE IF NOT EXISTS account(
   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   email TEXT NOT NULL,
   nickname TEXT NOT NULL,
   activated BOOLEAN NOT NULL,
   add_date TIMESTAMPTZ NOT NULL DEFAULT NOW()
   );