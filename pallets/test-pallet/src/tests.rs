// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the correct error is thrown on None value
		assert_noop!(
			TemplateModule::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
