#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_core::env::{ContractEnv, DefaultSrmlTypes, EnvTypes};
use parity_codec::{Codec, Decode, Encode};

type AccountId = <ContractEnv<DefaultSrmlTypes> as EnvTypes>::AccountId;
type Balance = <ContractEnv<DefaultSrmlTypes> as EnvTypes>::Balance;

pub mod default;
pub mod traits;

#[derive(Clone, Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Range<I: traits::SimpleArithmetic + traits::Member + Codec> {
    start: I,
    end: I,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct StateObject<T: traits::Member + Codec> {
    predicate: AccountId,
    data: T,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct StateUpdate<T: traits::Member + Codec, I: traits::SimpleArithmetic + traits::Member + Codec> {
    range: Range<I>,
    state_object: StateObject<T>,
    plasma_contract: AccountId,
    plasma_block_number: I,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Checkpoint<T: traits::Member + Codec, I: traits::SimpleArithmetic + traits::Member + Codec> {
    state_update: StateUpdate<T, I>,
    sub_range: Range<I>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Transaction<T: traits::Member + Codec, I: traits::SimpleArithmetic + traits::Member + Codec> {
    deposit_contract: AccountId,
    range: Range<I>,
    body: T,
}
