CREATE TABLE "public"."processed_blocks" ("contract" text NOT NULL, "chain_id" integer NOT NULL, "height" integer NOT NULL, PRIMARY KEY ("contract","chain_id","height") );
