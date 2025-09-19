diesel::table! {
    servers (name) {
        id -> BigSerial,
        name -> Text,
        version -> Text,
        difficulty -> Text,
        port -> BigInt,
        started -> Bool
    }
}
