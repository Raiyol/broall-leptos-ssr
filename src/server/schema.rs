// @generated automatically by Diesel CLI.

diesel::table! {
    bookmark (id_user, id_novel) {
        id_user -> Unsigned<Integer>,
        id_novel -> Unsigned<Integer>,
        chapter -> Unsigned<Integer>,
    }
}

diesel::table! {
    chapter (id) {
        id -> Unsigned<Integer>,
        id_novel -> Unsigned<Integer>,
        number -> Unsigned<Integer>,
        date -> Nullable<Datetime>,
        #[max_length = 255]
        title_en -> Nullable<Varchar>,
        #[max_length = 255]
        title_cn -> Nullable<Varchar>,
        content -> Nullable<Longtext>,
        dict -> Nullable<Longtext>,
    }
}

diesel::table! {
    comment (id) {
        id -> Unsigned<Integer>,
        id_chapter -> Unsigned<Integer>,
        id_user -> Unsigned<Integer>,
        #[max_length = 2048]
        text -> Varchar,
        date -> Nullable<Datetime>,
    }
}

diesel::table! {
    genre (id) {
        id -> Unsigned<Integer>,
        #[max_length = 30]
        name -> Varchar,
    }
}

diesel::table! {
    novel (id) {
        id -> Unsigned<Integer>,
        #[max_length = 50]
        url -> Varchar,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 30]
        cn_name -> Varchar,
        #[max_length = 20]
        author -> Nullable<Varchar>,
        summary -> Nullable<Text>,
        #[max_length = 50]
        img -> Nullable<Varchar>,
        date -> Nullable<Datetime>,
        completed -> Tinyint,
    }
}

diesel::table! {
    novel_genre (id_novel, id_genre) {
        id_novel -> Unsigned<Integer>,
        id_genre -> Unsigned<Integer>,
    }
}

diesel::table! {
    review (id) {
        id -> Unsigned<Integer>,
        id_novel -> Unsigned<Integer>,
        id_user -> Unsigned<Integer>,
        rate -> Nullable<Integer>,
        text -> Nullable<Text>,
        date -> Nullable<Datetime>,
    }
}

diesel::table! {
    review_user_liked (id_review, id_user) {
        id_review -> Unsigned<Integer>,
        id_user -> Unsigned<Integer>,
    }
}

diesel::table! {
    user (id) {
        id -> Unsigned<Integer>,
        #[max_length = 16]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 60]
        pwd -> Varchar,
        confirmed -> Tinyint,
        #[max_length = 5]
        role -> Nullable<Varchar>,
        #[max_length = 32]
        pfp -> Varchar,
        date -> Nullable<Datetime>,
    }
}

diesel::joinable!(bookmark -> novel (id_novel));
diesel::joinable!(bookmark -> user (id_user));
diesel::joinable!(chapter -> novel (id_novel));
diesel::joinable!(comment -> chapter (id_chapter));
diesel::joinable!(comment -> user (id_user));
diesel::joinable!(novel_genre -> genre (id_genre));
diesel::joinable!(novel_genre -> novel (id_novel));
diesel::joinable!(review -> novel (id_novel));
diesel::joinable!(review -> user (id_user));
diesel::joinable!(review_user_liked -> review (id_review));
diesel::joinable!(review_user_liked -> user (id_user));

diesel::allow_tables_to_appear_in_same_query!(
    bookmark,
    chapter,
    comment,
    genre,
    novel,
    novel_genre,
    review,
    review_user_liked,
    user,
);
