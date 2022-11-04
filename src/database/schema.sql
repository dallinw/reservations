CREATE TABLE "carriers"
(
    "abbreviation" text PRIMARY KEY,
    "name"         text NOT NULL,
    "created_at"   timestamptz DEFAULT (now())
);

CREATE TABLE "flights"
(
    "id"                    BIGSERIAL,
    "carrier"               text   NOT NULL,
    "first_class_seat_rows" bigint NOT NULL,
    "economy_seat_rows"     bigint NOT NULL,
    "width"                 bigint NOT NULL,
    PRIMARY KEY ("id", "carrier")
);

CREATE TABLE "schedules"
(
    "id"          BIGSERIAL PRIMARY KEY,
    "departure"   timestamptz NOT NULL,
    "arrival"     timestamptz NOT NULL,
    "source"      text        NOT NULL,
    "destination" text        NOT NULL,
    "flight"      bigint      NOT NULL,
    "carrier"     text        NOT NULL
);

CREATE TABLE "reservations"
(
    "id"       BIGSERIAL PRIMARY KEY,
    "schedule" bigint  NOT NULL,
    "user"     text    NOT NULL,
    "row"      bigint  NOT NULL,
    "seat"     char(1) NOT NULL
);

ALTER TABLE "flights"
    ADD FOREIGN KEY ("carrier") REFERENCES "carriers" ("abbreviation");

ALTER TABLE "schedules"
    ADD FOREIGN KEY ("flight", "carrier") REFERENCES "flights" ("id", "carrier");

ALTER TABLE "reservations"
    ADD FOREIGN KEY ("schedule") REFERENCES "schedules" ("id");
