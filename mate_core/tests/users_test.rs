use mate_core::models::users::UserBuilder;

#[test]
fn test_create_user_with_ulid() {
    let ulid = ulid::Ulid::new();
    // Assume a User struct with fields like id, name, and email.
    // The builder pattern is used here.
    let user = UserBuilder::default()
        .id(Some(ulid))
        .name("Alice".to_string())
        .build()
        .unwrap();

    assert_eq!(user.name, "Alice");
}

#[test]
fn test_create_user_without_ulid() {
    // The builder will generate a new ULID when `id` is not set.
    let user = UserBuilder::default()
        .name("Bob".to_string())
        .build()
        .unwrap();

    // println!("{}", user.id.to_string());
}