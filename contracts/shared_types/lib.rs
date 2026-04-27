#![no_std]

use soroban_sdk::{contracterror, contracttype, Address, String};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SwiftChainError {
    /// Caller is not authorized to perform the requested action.
    Unauthorized = 1,
    /// Contract or protocol state has already been initialized.
    AlreadyInitialized = 2,
    /// Contract or protocol state has not been initialized yet.
    NotInitialized = 3,
    /// Delivery record or related escrow entry could not be found.
    DeliveryNotFound = 4,
    /// Requested operation is invalid for the current protocol state.
    InvalidState = 5,
    /// Contract balance is too low to complete the requested transfer.
    InsufficientFunds = 6,
    /// Escrow funds are locked and cannot be released or refunded yet.
    EscrowLocked = 7,
    /// Delivery identifier already exists in protocol storage.
    DuplicateDelivery = 8,
    /// Provider or driver record could not be found.
    ProviderNotFound = 9,
    /// Address argument is invalid for the requested operation.
    InvalidAddress = 10,
}

// Event topic constants for on-chain event tracking
pub mod events {
    use soroban_sdk::{Env, Symbol};

    pub fn escrow_funded(env: &Env) -> Symbol {
        Symbol::new(env, "escrow_funded")
    }

    pub fn escrow_released(env: &Env) -> Symbol {
        Symbol::new(env, "escrow_released")
    }

    pub fn escrow_refunded(env: &Env) -> Symbol {
        Symbol::new(env, "escrow_refunded")
    }

    pub fn delivery_disputed(env: &Env) -> Symbol {
        Symbol::new(env, "delivery_disputed")
    }

    pub fn dispute_resolved(env: &Env) -> Symbol {
        Symbol::new(env, "dispute_resolved")
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeliveryStatus {
    Created,
    InTransit,
    Delivered,
    Disputed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryDetails {
    pub id: u64,
    pub driver: String,
    pub status: DeliveryStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowStatus {
    Pending,
    Released,
    Refunded,
    Disputed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EscrowRecord {
    pub sender: Address,
    pub driver: Address,
    pub token: Address,
    pub amount: i128,
    pub status: EscrowStatus,
}

#[cfg(test)]
mod test {
    use super::SwiftChainError;

    #[test]
    fn unauthorized_has_expected_discriminant() {
        assert_eq!(SwiftChainError::Unauthorized as u32, 1);
    }

    #[test]
    fn already_initialized_has_expected_discriminant() {
        assert_eq!(SwiftChainError::AlreadyInitialized as u32, 2);
    }

    #[test]
    fn not_initialized_has_expected_discriminant() {
        assert_eq!(SwiftChainError::NotInitialized as u32, 3);
    }

    #[test]
    fn delivery_not_found_has_expected_discriminant() {
        assert_eq!(SwiftChainError::DeliveryNotFound as u32, 4);
    }

    #[test]
    fn invalid_state_has_expected_discriminant() {
        assert_eq!(SwiftChainError::InvalidState as u32, 5);
    }

    #[test]
    fn insufficient_funds_has_expected_discriminant() {
        assert_eq!(SwiftChainError::InsufficientFunds as u32, 6);
    }

    #[test]
    fn escrow_locked_has_expected_discriminant() {
        assert_eq!(SwiftChainError::EscrowLocked as u32, 7);
    }

    #[test]
    fn duplicate_delivery_has_expected_discriminant() {
        assert_eq!(SwiftChainError::DuplicateDelivery as u32, 8);
    }

    #[test]
    fn provider_not_found_has_expected_discriminant() {
        assert_eq!(SwiftChainError::ProviderNotFound as u32, 9);
    }

    #[test]
    fn invalid_address_has_expected_discriminant() {
        assert_eq!(SwiftChainError::InvalidAddress as u32, 10);
    }
}
