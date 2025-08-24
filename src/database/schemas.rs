diesel::table! {
    servers (name) {
        name -> Text,
        adresse -> Nullable<Text>,
    }
}
