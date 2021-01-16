use crate::{mock::*};
use frame_support::{assert_ok};

const BOB: u64 = 2;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(CereDDCModule::send_data(Origin::signed(1), BOB, Vec::new()));
	});
}
