CREATE TABLE "carriers"
(
    "id"           BIGSERIAL PRIMARY KEY,
    "name"         text NOT NULL,
    "abbreviation" text NOT NULL,
    "created_at"   timestamptz DEFAULT (now())
);

CREATE TABLE "airplanes"
(
    "id"          BIGSERIAL PRIMARY KEY,
    "carrier"     bigint NOT NULL,
    "route"       text,
    "first_class" bigint NOT NULL,
    "economy"     bigint NOT NULL,
    "width"       bigint NOT NULL
);

CREATE TABLE "schedules"
(
    "id"          BIGSERIAL PRIMARY KEY,
    "departure"   timestamptz,
    "arrival"     timestamptz,
    "source"      text NOT NULL,
    "destination" text NOT NULL,
    "carrier"     bigint,
    "route"       bigint,
    "airplane"    bigint
);

CREATE TABLE "reservations"
(
    "id"     BIGSERIAL PRIMARY KEY,
    "flight" bigint  NOT NULL,
    "user"   text    NOT NULL,
    "row"    bigint  NOT NULL,
    "seat"   char(1) NOT NULL
);

ALTER TABLE "airplanes"
    ADD FOREIGN KEY ("carrier") REFERENCES "carriers" ("id");

ALTER TABLE "schedules"
    ADD FOREIGN KEY ("airplane") REFERENCES "airplanes" ("id");

ALTER TABLE "reservations"
    ADD FOREIGN KEY ("flight") REFERENCES "schedules" ("id");
