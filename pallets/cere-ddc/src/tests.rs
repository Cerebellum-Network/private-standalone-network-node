use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

const BOB: u64 = 2;

#[test]
fn send_data_works_valid_input() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(CereDDCModule::send_data(Origin::signed(1), BOB, b"12345678".to_vec()));
	});
}

#[test]
fn send_data_error_too_long() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			CereDDCModule::send_data(Origin::signed(1), BOB, b"TestTooLongString".to_vec()),
			Error::<Test>::TooLong
		);
	});
}

#[test]
fn send_data_error_too_short() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			CereDDCModule::send_data(Origin::signed(1), BOB, b"Short".to_vec()),
			Error::<Test>::TooShort
		);
	});
}
