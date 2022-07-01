
-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."SpawnFilter" add column "child_salt" bytea
--  not null;

alter table "public"."SpawnFilter" rename column "assets" to "assets_";

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."SpawnFilter" add column "assets_" jsonb
--  not null;

alter table "public"."SpawnFilter" add constraint "SpawnFilter_spawn_assets_id_one_to_many_key" unique (spawn_assets_id_one_to_many);
alter table "public"."SpawnFilter" alter column "spawn_assets_id_one_to_many" drop not null;
alter table "public"."SpawnFilter" add column "spawn_assets_id_one_to_many" int4;

alter table "public"."SpawnFilter" alter column "asset_amt" drop not null;
alter table "public"."SpawnFilter" add column "asset_amt" int8;

alter table "public"."SpawnFilter" alter column "asset_id" drop not null;
alter table "public"."SpawnFilter" add column "asset_id" int4;

alter table "public"."spf_to_assets" drop constraint "spf_to_assets_spf_id_assets_id_key";

alter table "public"."SpawnFilter" drop constraint "SpawnFilter_spawn_assets_id_one_to_many_key";

DROP TABLE "public"."spf_to_assets";

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."SpawnFilter" add column "spawn_assets_id_one_to_many" integer
--  not null;

DROP TABLE "public"."spawn_filter_assets";

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."SpawnFilter" add column "origin_address" bytea
--  not null;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."SpawnFilter" add column "origin_network_id" integer
--  not null;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."SpawnFilter" add column "salt" bytea
--  not null;

alter table "public"."SpawnFilter" alter column "salt" drop not null;
alter table "public"."SpawnFilter" add column "salt" text;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."SpawnFilter" add column "program" text
--  not null;

alter table "public"."SpawnFilter" alter column "program" drop not null;
alter table "public"."SpawnFilter" add column "program" text;

DROP TABLE "public"."SucceededFilter";

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."TransferFilter" add column "to" bytea
--  not null;

alter table "public"."TransferFilter" alter column "to" drop not null;
alter table "public"."TransferFilter" add column "to" text;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."TransferFilter" add column "origin_address" bytea
--  not null;

alter table "public"."TransferFilter" alter column "origin_address" drop not null;
alter table "public"."TransferFilter" add column "origin_address" text;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."TransferFilter" add column "from" bytea
--  not null;

alter table "public"."TransferFilter" alter column "from" drop not null;
alter table "public"."TransferFilter" add column "from" text;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."TransferFilter" add column "salt" bytea
--  not null;

alter table "public"."TransferFilter" alter column "salt" drop not null;
alter table "public"."TransferFilter" add column "salt" text;

alter table "public"."SpawnFilter" alter column "tag" drop not null;
alter table "public"."SpawnFilter" add column "tag" text;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."CallFilter" add column "origin_address" bytea
--  not null;

alter table "public"."CallFilter" alter column "origin_address" drop not null;
alter table "public"."CallFilter" add column "origin_address" text;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."CallFilter" add column "to" bytea
--  not null;

alter table "public"."CallFilter" alter column "to" drop not null;
alter table "public"."CallFilter" add column "to" text;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."CallFilter" add column "salt" bytea
--  not null;

alter table "public"."CallFilter" alter column "salt" drop not null;
alter table "public"."CallFilter" add column "salt" text;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."TransferFilter" add column "salt" Text
--  not null;

alter table "public"."TransferFilter" rename column "asset_amount" to "asset_amt";

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."TransferFilter" add column "from" text
--  not null;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."TransferFilter" add column "origin_address" text
--  not null;

alter table "public"."TransferFilter" alter column "tag" drop not null;
alter table "public"."TransferFilter" add column "tag" text;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."TransferFilter" add column "origin_network_id" integer
--  not null;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."CallFilter" add column "origin_address" text
--  not null;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."CallFilter" add column "origin_network_id" integer
--  not null;

-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."CallFilter" add column "salt" text
--  not null;

alter table "public"."CallFilter" alter column "tag" drop not null;
alter table "public"."CallFilter" add column "tag" text;

ALTER TABLE "public"."TransferFilter" ALTER COLUMN "asset_amt" TYPE integer;

DROP TABLE "public"."SpawnFilter";

DROP TABLE "public"."CallFilter";

DROP TABLE "public"."TransferFilter";
