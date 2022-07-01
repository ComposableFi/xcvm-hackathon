alter table "public"."queue" add column "salt" bytea
 not null default ''::bytea;
