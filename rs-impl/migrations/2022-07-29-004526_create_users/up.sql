CREATE TABLE "comics" (
	"id" serial,
	"username" varchar NOT NULL,
	"title" varchar NOT NULL,
	"issue_number" varchar NOT NULL,
	"main_character" varchar NOT NULL,
	"genre" varchar NOT NULL,
	"cover_year" date NOT NULL,
	"publisher" varchar NOT NULL,
	"grade" float NOT NULL,
	"price" float NOT NULL,
	"image_url" varchar NOT NULL,
	"created_at" timestamp NOT NULL DEFAULT NOW(),
	"updated_at" timestamp NOT NULL DEFAULT NOW(),
	PRIMARY KEY ("id")
);
