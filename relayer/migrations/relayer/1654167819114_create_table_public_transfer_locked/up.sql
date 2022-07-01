CREATE TABLE "public"."transfer_locked" ("id" bytea NOT NULL, PRIMARY KEY ("id") , FOREIGN KEY ("id") REFERENCES "public"."transfer"("id") ON UPDATE cascade ON DELETE cascade);
