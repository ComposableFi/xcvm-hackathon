CREATE TABLE "public"."queue" ("id" serial NOT NULL, "source" integer NOT NULL, "destination" integer NOT NULL, "payload" bytea NOT NULL, "caller" text NOT NULL, PRIMARY KEY ("id") );
