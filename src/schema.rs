table! {
    minions (id) {
        id -> Integer,
        name -> Text,
        active -> Bool,
        key -> Nullable<Text>,
        ip -> Nullable<Text>,
        port -> Nullable<Integer>,
        username -> Nullable<Text>,
        directory -> Nullable<Text>,
    }
}
