alter table "public"."transfer" alter column "remote_asset_id" drop not null;
alter table "public"."transfer" add column "remote_asset_id" bytea;
