table! {
    contact (id) {
        id -> Int4,
        uuid -> Text,
        name -> Text,
        address -> Nullable<Text>,
        zipcode -> Nullable<Text>,
        city -> Nullable<Text>,
        country -> Nullable<Text>,
    }
}
