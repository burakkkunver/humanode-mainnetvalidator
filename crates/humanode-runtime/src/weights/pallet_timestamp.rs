// DO NOT EDIT!
//! Autogenerated weights for `pallet_timestamp`

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_timestamp`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_timestamp::WeightInfo for WeightInfo<T> {
  fn set() -> Weight {
    // Minimum execution time: 11_000 nanoseconds.
    Weight::from_ref_time(11_000_000)
      .saturating_add(T::DbWeight::get().reads(2))
      .saturating_add(T::DbWeight::get().writes(1))
  }
  fn on_finalize() -> Weight {
    // Minimum execution time: 4_000 nanoseconds.
    Weight::from_ref_time(4_000_000)
  }
}
