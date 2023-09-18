/// Doc comments
/// written in markdown & support code blocks

pub struct SavingsAccount {
    balance: i32,
}

impl  SavingsAccount {
    /// Creates `SavingsAccount` with balance of 0
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use savings:SavingsAccount;
    /// 
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    pub fn new() -> SavingsAccount {
        SavingsAccount {
            balance: 0,
        }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amt: i32) {
        if amt < 0 {
            panic!("Negative num not allowed");
        }
        self.balance += amt
    }

    pub fn transfer(&self, acct_num:i32, amt:i32 ) -> Result<String, String> {
        Ok(format!("transferred {amt} to {acct_num}"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_have_zero_starting() {
        let acct = SavingsAccount::new();
        let balance = acct.get_balance();
        assert_eq!(balance, 0);
    }

    #[test]
    fn can_deposit() {
        let mut acct: SavingsAccount = SavingsAccount::new();
        acct.deposit(5);
        assert_eq!(acct.get_balance(), 5);
    }

    #[test]
    #[should_panic]
    fn panic_on_negative_deposit() {
        let mut acct = SavingsAccount::new();
        acct.deposit(-5);

    }

    #[test]
    fn should_transfer() -> Result<(), String> {
        let mut acct = SavingsAccount::new();
        acct.deposit(100);
        acct.transfer(12, 500)?;
        Ok(())
    }
}