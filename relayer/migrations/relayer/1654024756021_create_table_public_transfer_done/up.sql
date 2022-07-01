CREATE TABLE "public"."transfer_done" ("id" bytea NOT NULL, PRIMARY KEY ("id") , FOREIGN KEY ("id") REFERENCES "public"."transfer"("id") ON UPDATE cascade ON DELETE cascade, UNIQUE ("id"));
