use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use pallet_timestamp::{self as timestamp};

// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
		
// 		// Dispatch a signed extrinsic.
// 		// { dna: 0xbb1370e1cfbdd2a4c5e943d488516dc0fc65258b376a86d8ee12eebc8e7f2d6e, price: 0, gender: Male, owner: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 (5FHneW46...), create: 1658928192004 } 
// 		assert_ok!(KittiesModule::create_kitty(Origin::signed(1), b"Test".to_vec()));
// 		// Read pallet storage and assert an expected result.
// 		// assert_eq!(KittiesModule::kitty_id(), Some(42));
// 		// assert_eq!(KittiesModule::get_kitty(), Some(42));
// 		// assert_eq!(KittiesModule::kitty_owned(), Some(42));
// 	})
// }

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(KittiesModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
