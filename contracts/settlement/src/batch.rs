use crate::{CalloraSettlement, MAX_BATCH_SIZE, SettlementError};
use soroban_sdk::{contracttype, Address, Env, Vec};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct SettleInput {
    pub developer: Address,
    pub amount: i128,
    pub to: Option<Address>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum SettleOutcome {
    Success,
    AmountNotPositive,
    ClaimWindowClosed,
    InsufficientBalance,
    DailyWithdrawCapExceeded,
    DeveloperBalanceUnderflow,
    OtherError,
}

impl From<SettlementError> for SettleOutcome {
    fn from(err: SettlementError) -> Self {
        match err {
            SettlementError::AmountNotPositive => SettleOutcome::AmountNotPositive,
            SettlementError::ClaimWindowClosed => SettleOutcome::ClaimWindowClosed,
            SettlementError::InsufficientDeveloperBalance => {
                SettleOutcome::InsufficientBalance
            }
            SettlementError::DailyWithdrawCapExceeded => {
                SettleOutcome::DailyWithdrawCapExceeded
            }
            SettlementError::DeveloperBalanceUnderflow => {
                SettleOutcome::DeveloperBalanceUnderflow
            }
            _ => SettleOutcome::OtherError,
        }
    }
}

/// Execute a batch of settlement operations.
///
/// Batch-level validation errors are returned before any per-item processing.
/// Individual settlement errors are converted into `SettleOutcome` values so
/// that each input receives one corresponding outcome.
pub fn batch_settle(
    env: &Env,
    settlements: Vec<SettleInput>,
) -> Result<Vec<SettleOutcome>, SettlementError> {
    let mut outcomes = Vec::new(env);

    if settlements.is_empty() {
        return Err(SettlementError::BatchEmpty);
    }

    if settlements.len() > MAX_BATCH_SIZE {
        return Err(SettlementError::BatchTooLarge);
    }

    // All items in a batch must belong to the same developer/tenant.
    let claimant = settlements.get_unchecked(0).developer.clone();

    for input in settlements.iter() {
        if input.developer != claimant {
            return Err(SettlementError::CrossTenantBatch);
        }
    }

    for input in settlements.iter() {
        let res = CalloraSettlement::withdraw_developer_balance(
            env.clone(),
            input.developer.clone(),
            input.amount,
            input.to.clone(),
        );

        match res {
            Ok(_) => outcomes.push_back(SettleOutcome::Success),
            Err(e) => outcomes.push_back(e.into()),
        }
    }

    Ok(outcomes)
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, Vec};

    fn make_input(env: &Env, dev: Address) -> SettleInput {
        SettleInput {
            developer: dev,
            amount: 100,
            to: None,
        }
    }

    #[test]
    fn test_batch_settle_empty_returns_error() {
        let env = Env::default();
        let settlements = Vec::new(&env);

        let result = batch_settle(&env, settlements);

        assert!(result.is_err());
        assert_eq!(result.unwrap_error(), SettlementError::BatchEmpty);
    }

    #[test]
    fn test_batch_settle_cap_enforced() {
        let env = Env::default();
        let mut settlements = Vec::new(&env);

        for _ in 0..(MAX_BATCH_SIZE + 1) {
            settlements.push_back(make_input(
                &env,
                Address::generate(&env),
            ));
        }

        let result = batch_settle(&env, settlements);

        assert!(result.is_err());
        assert_eq!(result.unwrap_error(), SettlementError::BatchTooLarge);
    }

    #[test]
    fn test_batch_settle_cross_tenant_error() {
        let env = Env::default();
        let mut settlements = Vec::new(&env);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        settlements.push_back(make_input(&env, alice));
        settlements.push_back(make_input(&env, bob));

        let result = batch_settle(&env, settlements);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_error(),
            SettlementError::CrossTenantBatch
        );
    }
}
