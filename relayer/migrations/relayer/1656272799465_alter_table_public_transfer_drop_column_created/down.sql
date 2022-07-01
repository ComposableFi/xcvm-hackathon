alter table "public"."transfer" alter column "created" drop not null;
alter table "public"."transfer" add column "created" timestamptz;
