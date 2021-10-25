-- Add migration script here
-- Add migration script here
-- Add migration script here
-- CREATE OR REPLACE FUNCTION trigger_set_timestamp()
-- RETURNS TRIGGER AS $$
-- BEGIN
--   NEW.updated_at = NOW();
--   RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql;


-- create function upd_timestamp() returns trigger as
-- $$
-- begin
--   new.modified = current_timestamp;
--   return new;
-- end
-- $$
-- language plpgsql;

create extension "uuid-ossp";
CREATE TABLE IF NOT EXISTS rssfeed(
   account_id UUID NOT NULL,
   link TEXT NOT NULL,
   add_date TIMESTAMPTZ NOT NULL DEFAULT NOW()
   );

-- CREATE TRIGGER set_timestamp
-- BEFORE INSERT ON rssfeed
-- FOR EACH ROW
-- EXECUTE PROCEDURE trigger_set_timestamp();


-- create trigger t_name before insert on "account" for each row execute procedure upd_timestamp();