use diesel::*;

table! {
    addresses (id, user_id) {
        id -> Int4,
        user_id -> Int4,
        door_number -> Nullable<Int4>,
        postcode -> Nullable<Varchar>,
        description -> Nullable<Property_type>,
        created_at -> Nullable<Int8>,
        updated_at -> Nullable<Int8>,
    }
}

table! {
    claims (id, user_id) {
        id -> Int4,
        user_id -> Int4,
        against -> Nullable<Loss_damage_type>,
        date_of_incident -> Nullable<Int8>,
        structure_affected -> Nullable<Yn_type>,
        created_at -> Nullable<Int8>,
        updated_at -> Nullable<Int8>,
    }
}

table! {
    documents (id, claim_id) {
        id -> Int4,
        claim_id -> Int4,
        url -> Nullable<Text>,
        file_name -> Nullable<Text>,
        description -> Nullable<Text>,
        created_at -> Nullable<Int8>,
        updated_at -> Nullable<Int8>,
    }
}

table! {
    identity (id, user_id) {
        id -> Int4,
        user_id -> Int4,
        is_a -> Nullable<Claiment_type>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        created_at -> Nullable<Int8>,
        updated_at -> Nullable<Int8>,
    }
}

table! {
    phone_numbers (id, identity_id) {
        id -> Int4,
        identity_id -> Int4,
        is_a -> Nullable<P_number_type>,
        name -> Nullable<Varchar>,
        number -> Nullable<Int4>,
        extention -> Nullable<Int2>,
        calling_code -> Nullable<Int2>,
        created_at -> Nullable<Int8>,
        updated_at -> Nullable<Int8>,
    }
}

table! {
    users (id) {
        id -> Int4,
        created_at -> Nullable<Int8>,
        updated_at -> Nullable<Int8>,
    }
}

joinable!(addresses -> users (user_id));
joinable!(claims -> users (user_id));
joinable!(identity -> users (user_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    claims,
    documents,
    identity,
    phone_numbers,
    users,
);
