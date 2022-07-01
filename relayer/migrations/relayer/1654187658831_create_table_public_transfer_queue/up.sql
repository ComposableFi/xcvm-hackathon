CREATE TABLE "public"."transfer_queue" ("transfer_id" bytea NOT NULL, "queue_id" integer NOT NULL, PRIMARY KEY ("transfer_id") , FOREIGN KEY ("transfer_id") REFERENCES "public"."transfer"("id") ON UPDATE cascade ON DELETE cascade, FOREIGN KEY ("queue_id") REFERENCES "public"."queue"("id") ON UPDATE cascade ON DELETE cascade);
