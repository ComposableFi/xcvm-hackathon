alter table "public"."transfer_queue"
  add constraint "transfer_queue_transfer_id_fkey"
  foreign key ("transfer_id")
  references "public"."transfer"
  ("id") on update cascade on delete cascade;
