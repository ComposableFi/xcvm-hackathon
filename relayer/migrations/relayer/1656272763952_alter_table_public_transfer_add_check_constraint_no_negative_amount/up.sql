alter table "public"."transfer" add constraint "no_negative_amount" check (amount >= 0);
