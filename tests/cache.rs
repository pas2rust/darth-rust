mod config;
pub use config::User;

// #[test]
// fn test_vec_update() {
// let mut cache = User::vec_new_cache();
// let mut user2 = User::default();
// user2.set_name("New Name");
// user2.set_id("1");
// user2.vec_insert_cache(&mut cache);
// user2.vec_update_by_id_in_cache(user2.clone(), &mut cache);
// user2.print_err("");
// assert_eq!(user2.vec_find_by_id_in_cache(&cache), Some(&user2),);
// }
//
// #[test]
// fn test_hash_update() {
// let mut cache = User::hash_new_cache();
// let user1 = User::default();
// let mut user2 = User::default();
// user2.set_name("New Name");
// user2.set_id("1");
// user1.hash_set_cache("user1", &mut cache);
// user2.hash_set_cache("user1", &mut cache);
// user2.print_err("");
// assert_eq!(User::hash_find_by_key_in_cache("user1", &cache), Some(&user2));
// }
// #[test]
// fn test_vec_find_many() {
// let mut cache = User::vec_new_cache();
// let mut user1 = User::default();
// user1.set_name("Same Name");
// user1.set_id("1");
// let mut user2 = user1.clone();
// user2.set_id("2");
// user1.vec_insert_cache(&mut cache);
// user2.vec_insert_cache(&mut cache);
// let found_users = User::vec_find_many_by_name_in_cache(&user1, &cache);
// user1.print_err("");
// user2.print_err("");
// assert_eq!(found_users, vec![&user1, &user2]);
// }
//
// #[test]
// fn test_hash_find_many() {
// let mut cache = User::hash_new_cache();
// let user1 = User::default();
// let user2 = User::default();
// user1.hash_set_cache("user1", &mut cache);
// user2.hash_set_cache("user2", &mut cache);
// let found_users =
// User::hash_find_by_keys_in_cache(vec!["user1", "user2"], &cache);
// user1.print_err("");
// user2.print_err("");
// assert_eq!(found_users, vec![Some(&user1), Some(&user2)]);
// }
