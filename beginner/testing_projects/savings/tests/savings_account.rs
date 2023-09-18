use savings::SavingsAccount;
mod utils;

#[test]
fn should_have_zero_starting() {
    utils::common_setup();
    let acct = SavingsAccount::new();
    assert_eq!(acct.get_balance(), 0);
}