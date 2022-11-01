table! {
    comics (id) {
        id -> Int4,
        username -> Varchar,
        title -> Varchar,
        issue_number -> Varchar,
        main_character -> Varchar,
        genre -> Varchar,
        cover_year -> Date,
        publisher -> Varchar,
        grade -> Float8,
        price -> Float8,
        image_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
