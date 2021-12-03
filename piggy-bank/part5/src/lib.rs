//! dcb bank smart contract.
//!
//! Allows anyone to insert GTU, but only the owner can "smash" it and
//! retrieve the GTU. Prevents more GTU to be inserted after being smashed.
//!
//! This smart contract module is developed as part of a upcoming tutorial on
//! developing smart contracts.
//!
//! Covers:
//! - Reading owner, sender and self_balance from the context.
//! - The `ensure` macro.
//! - The `payable` attribute.
//! - Creating a simple transfer action.

// Pulling in everything from the smart contract standard library.
use concordium_std::*;

/// The state of the dcb bank
#[derive(Debug, Serialize, PartialEq, Eq)]
enum DCBBankState {
    /// Alive and well, allows for GTU to be inserted.
    Intact,
    /// The dcb bank has been emptied, preventing further GTU to be inserted.
    Smashed,
}

/// Setup a new Intact dcb bank.
#[init(contract = "DCBBank")]
fn dcb_init(_ctx: &impl HasInitContext) -> InitResult<DCBBankState> {
    // Always succeeds
    Ok(DCBBankState::Intact)
}

/// Insert some GTU into a dcb bank, allowed by anyone.
#[receive(contract = "DCBBank", name = "insertAmount", payable)]
fn dcb_insert<A: HasActions>(
    _ctx: &impl HasReceiveContext,
    _amount: Amount,
    state: &mut DCBBankState,
) -> ReceiveResult<A> {
    // Ensure the dcb bank has not been smashed already.
    ensure!(*state == DCBBankState::Intact);
    // Just accept since the GTU balance is managed by the chain.
    Ok(A::accept())
}

/// Smash a dcb bank retrieving the GTU, only allowed by the owner.
#[receive(contract = "DCBBank", name = "smashAmount")]
fn dcb_smash<A: HasActions>(
    ctx: &impl HasReceiveContext,
    state: &mut DCBBankState,
) -> ReceiveResult<A> {
    // Get the contract owner, i.e. the account who initialized the contract.
    let owner = ctx.owner();
    // Get the sender, who triggered this function, either a smart contract or
    // an account.
    let sender = ctx.sender();

    // Ensure only the owner can smash the dcb bank.
    ensure!(sender.matches_account(&owner));
    // Ensure the dcb bank has not been smashed already.
    ensure!(*state == DCBBankState::Intact);
    // Set the state to be smashed.
    *state = DCBBankState::Smashed;

    // Get the current balance of the smart contract.
    let balance = ctx.self_balance();
    // Result in a transfer of the whole balance to the contract owner.
    Ok(A::simple_transfer(&owner, balance))
}

/// Get the balance of the dcb bank, only allowed by the owner.
#[receive(contract = "DCBBank", name = "balanceOf")]
fn dcb_balance_of<A: HasActions> (
    ctx: &impl HasReceiveContext,
    _state: &mut DCBBankState,
) -> ReceiveResult<A> { 
    // Get the contract owner, i.e. the account who initialized the contract.
    let owner = ctx.owner();
    // Get the sender, who triggered this function, either a smart contract or
    // an account.
    let sender = ctx.sender();

    // Ensure only the owner can smash the dcb bank.
    ensure!(sender.matches_account(&owner));

    // Get the current balance of the smart contract.
    let balance = ctx.self_balance();
    // Result in a transfer of the whole balance to the contract owner.
    Ok(A::simple_transfer(&owner, balance))
}