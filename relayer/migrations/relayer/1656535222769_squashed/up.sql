
CREATE TABLE "public"."TransferFilter" ("tag" text NOT NULL, "to" text NOT NULL, "asset_id" Integer NOT NULL, "asset_amt" Integer NOT NULL, "id" serial NOT NULL, PRIMARY KEY ("id") , UNIQUE ("id"));

CREATE TABLE "public"."CallFilter" ("tag" text NOT NULL, "to" text NOT NULL, "payload" jsonb NOT NULL, "id" serial NOT NULL, PRIMARY KEY ("id") , UNIQUE ("id"));

CREATE TABLE "public"."SpawnFilter" ("tag" text NOT NULL, "network" int8 NOT NULL, "salt" text NOT NULL, "asset_id" integer NOT NULL, "asset_amt" bigint NOT NULL, "program" text NOT NULL, "id" serial NOT NULL, PRIMARY KEY ("id") , UNIQUE ("id"));

ALTER TABLE "public"."TransferFilter" ALTER COLUMN "asset_amt" TYPE int8;

alter table "public"."CallFilter" drop column "tag" cascade;

alter table "public"."CallFilter" add column "salt" text
 not null;

alter table "public"."CallFilter" add column "origin_network_id" integer
 not null;

alter table "public"."CallFilter" add column "origin_address" text
 not null;

alter table "public"."TransferFilter" add column "origin_network_id" integer
 not null;

alter table "public"."TransferFilter" drop column "tag" cascade;

alter table "public"."TransferFilter" add column "origin_address" text
 not null;

alter table "public"."TransferFilter" add column "from" text
 not null;

alter table "public"."TransferFilter" rename column "asset_amt" to "asset_amount";

alter table "public"."TransferFilter" add column "salt" Text
 not null;

alter table "public"."CallFilter" drop column "salt" cascade;

alter table "public"."CallFilter" add column "salt" bytea
 not null;

alter table "public"."CallFilter" drop column "to" cascade;

alter table "public"."CallFilter" add column "to" bytea
 not null;

alter table "public"."CallFilter" drop column "origin_address" cascade;

alter table "public"."CallFilter" add column "origin_address" bytea
 not null;

alter table "public"."SpawnFilter" drop column "tag" cascade;

alter table "public"."TransferFilter" drop column "salt" cascade;

alter table "public"."TransferFilter" add column "salt" bytea
 not null;

alter table "public"."TransferFilter" drop column "from" cascade;

alter table "public"."TransferFilter" add column "from" bytea
 not null;

alter table "public"."TransferFilter" drop column "origin_address" cascade;

alter table "public"."TransferFilter" add column "origin_address" bytea
 not null;

alter table "public"."TransferFilter" drop column "to" cascade;

alter table "public"."TransferFilter" add column "to" bytea
 not null;

CREATE TABLE "public"."SucceededFilter" ("origin_network_id" integer NOT NULL, "origin_address" bytea NOT NULL, "salt" bytea NOT NULL, "id" serial NOT NULL, PRIMARY KEY ("id") );

alter table "public"."SpawnFilter" drop column "program" cascade;

alter table "public"."SpawnFilter" add column "program" text
 not null;

alter table "public"."SpawnFilter" drop column "salt" cascade;

alter table "public"."SpawnFilter" add column "salt" bytea
 not null;

alter table "public"."SpawnFilter" add column "origin_network_id" integer
 not null;

alter table "public"."SpawnFilter" add column "origin_address" bytea
 not null;

CREATE TABLE "public"."spawn_filter_assets" ("asset_id" integer NOT NULL, "amount" int8 NOT NULL, "id" serial NOT NULL, PRIMARY KEY ("id") );

alter table "public"."SpawnFilter" add column "spawn_assets_id_one_to_many" integer
 not null;

CREATE TABLE "public"."spf_to_assets" ("spf_id" integer NOT NULL, "assets_id" integer NOT NULL, PRIMARY KEY ("spf_id","assets_id") , FOREIGN KEY ("spf_id") REFERENCES "public"."SpawnFilter"("id") ON UPDATE restrict ON DELETE restrict, FOREIGN KEY ("assets_id") REFERENCES "public"."spawn_filter_assets"("id") ON UPDATE restrict ON DELETE restrict);

alter table "public"."SpawnFilter" add constraint "SpawnFilter_spawn_assets_id_one_to_many_key" unique ("spawn_assets_id_one_to_many");

alter table "public"."spf_to_assets" add constraint "spf_to_assets_spf_id_assets_id_key" unique ("spf_id", "assets_id");

alter table "public"."SpawnFilter" drop column "asset_id" cascade;

alter table "public"."SpawnFilter" drop column "asset_amt" cascade;

alter table "public"."SpawnFilter" drop column "spawn_assets_id_one_to_many" cascade;

alter table "public"."SpawnFilter" add column "assets_" jsonb
 not null;

alter table "public"."SpawnFilter" rename column "assets_" to "assets";

alter table "public"."SpawnFilter" add column "child_salt" bytea
 not null;
