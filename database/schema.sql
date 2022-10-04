--  Note a lot of this is for demo purposes only. The airplane management system is external, as is the scheduled. We
--  need it here to seed the data however.

CREATE TABLE "reservations"
(
    "id"         BIGSERIAL PRIMARY KEY,
    "seat"       text   NOT NULL,
    "row"        bigint NOT NULL,
    "route"      text   NOT NULL,
    "created_at" timestampz DEFAULT (now())
);

CREATE TABLE "carriers"
(
    "abbreviation" text PRIMARY KEY,
    "name"         text NOT NULL,
    "description"  text NOT NULL
);

CREATE TABLE "airplanes"
(
    "id"      BIGSERIAL PRIMARY KEY,
    "carrier" text   NOT NULL,
    "rows"    bigint NOT NULL,
    "width"   bigint NOT NULL
);

CREATE TABLE "routes"
(
    "id"          BIGSERIAL PRIMARY KEY,
    "source"      text       NOT NULL,
    "destination" text       NOT NULL,
    "scheduled"   timestampz NOT NULL,
    "airplane"    bigint     NOT NULL
);

ALTER TABLE "routes"
    ADD FOREIGN KEY ("id") REFERENCES "reservations" ("id");

ALTER TABLE "airplanes"
    ADD FOREIGN KEY ("carrier") REFERENCES "carriers" ("abbreviation");

CREATE TABLE "airplanes_routes"
(
    "airplanes_id"    bigint NOT NULL,
    "routes_airplane" bigint NOT NULL,
    PRIMARY KEY ("airplanes_id", "routes_airplane")
);

ALTER TABLE "airplanes_routes"
    ADD FOREIGN KEY ("airplanes_id") REFERENCES "airplanes" ("id");

ALTER TABLE "airplanes_routes"
    ADD FOREIGN KEY ("routes_airplane") REFERENCES "routes" ("airplane");

