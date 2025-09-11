diesel::table! {
    servers (name) {
        name -> Text,
        version -> Text,
        difficulty -> Text,
        port -> BigInt,
        started -> Bool
    }
}
