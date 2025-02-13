#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet_3vm_evm_primitives::{
    Context, IsPrecompileResult, LinearCostPrecompile, Precompile, PrecompileHandle,
    PrecompileResult, PrecompileSet,
};

use frame_support::traits::Currency;
use pallet_3vm_evm_primitives::{ExitError, PrecompileFailure};
pub use pallet_evm_precompile_modexp::Modexp;
pub use pallet_evm_precompile_sha3fips::{Sha3FIPS256, Sha3FIPS512};
pub use pallet_evm_precompile_simple::{
    ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256,
};

use portal_precompile::PortalPrecompile;
use precompile_util_solidity::data::EvmData;
use sp_core::H160;
use sp_std::{collections::btree_map::BTreeMap, marker::PhantomData, vec::Vec};
use tokens_precompile::TokensPrecompile;
use vacuum_precompile::VacuumPrecompile;

pub mod precompile_mock;

pub const TOKENS_PRECOMPILE_PREFIX: &[u8] = &[9u8; 16];

fn hash(a: &u64) -> H160 {
    H160::from_low_u64_be(*a)
}

// Precompile set that will be used in production. Allows
#[derive(Debug, Default, Clone, Copy)]
pub struct T3rnPrecompiles<T>(PhantomData<T>);

impl<T> T3rnPrecompiles<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<T> PrecompileSet for T3rnPrecompiles<T>
where
    T: pallet_3vm_evm::Config
        + pallet_assets::Config
        + pallet_balances::Config
        + frame_system::Config,
    <T as pallet_assets::Config>::AssetId: From<u32>,
    <T as pallet_assets::Config>::AssetIdParameter: From<u32>,
    <T as pallet_assets::Config>::Balance: EvmData,
    <<T as pallet_3vm_evm::Config>::Currency as Currency<
        <T as frame_system::pallet::Config>::AccountId,
    >>::Balance: EvmData,
    sp_core::U256: From<<T as pallet_assets::Config>::Balance>,
    sp_core::U256: From<
        <<T as pallet_3vm_evm::Config>::Currency as Currency<
            <T as frame_system::pallet::Config>::AccountId,
        >>::Balance,
    >,
    TokensPrecompile<T>: Precompile,
    PortalPrecompile<T>: Precompile,
    VacuumPrecompile<T>: Precompile,
{
    fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
        let address = handle.code_address();
        match address {
            // Ethereum precompiles
            a if H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]) == a =>
                Some(<ECRecover as Precompile>::execute(handle)),
            a if H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]) == a =>
                Some(<Sha256 as Precompile>::execute(handle)),
            a if H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3]) == a =>
                Some(<Ripemd160 as Precompile>::execute(handle)),
            a if H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4]) == a =>
                Some(<Identity as Precompile>::execute(handle)),
            a if H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5]) == a =>
                Some(<Modexp as Precompile>::execute(handle)),
            // Non-Ethereum precompiles
            a if H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101]) == a =>
                Some(<Sha3FIPS256 as Precompile>::execute(handle)),
            a if H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 102]) == a =>
                Some(<Sha3FIPS512 as Precompile>::execute(handle)),
            a if H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 103]) == a =>
                Some(<ECRecoverPublicKey as Precompile>::execute(handle)),
            // t3rn precompiles
            a if H160([7u8; 20]) == a => Some(PortalPrecompile::<T>::execute(handle)),
            a if H160([8u8; 20]) == a => Some(VacuumPrecompile::<T>::execute(handle)),
            a if &a.to_fixed_bytes()[0..16] == TOKENS_PRECOMPILE_PREFIX =>
                Some(TokensPrecompile::<T>::execute(handle)),
            // Default
            _ => None,
        }
    }

    fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
        let mut is_precompile_result: bool = false;
        if (address == hash(&1)
            || address == hash(&2)
            || address == hash(&3)
            || address == hash(&4)
            || address == hash(&5)
            || address == hash(&101)
            || address == hash(&102)
            || address == hash(&103)
            || address == hash(&10001)
            || &address.to_fixed_bytes()[0..16] == TOKENS_PRECOMPILE_PREFIX)
        {
            is_precompile_result = true;
        }
        IsPrecompileResult::Answer {
            is_precompile: is_precompile_result,
            extra_cost: 0,
        }
    }
}
