alter table "public"."transfer" add constraint "no_negative_asset_id" check (asset_id >= 0);
