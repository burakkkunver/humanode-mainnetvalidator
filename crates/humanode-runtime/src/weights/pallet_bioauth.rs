// DO NOT EDIT!
//! Autogenerated weights for `pallet_bioauth`

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bioauth`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bioauth::WeightInfo for WeightInfo<T> {
  /// The range of component `n` is `[0, 30719999]`.
  /// The range of component `a` is `[0, 3071]`.
  fn authenticate(_a: u32, n: u32, ) -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `183 + a * (40 ±0) + n * (19 ±0)`
    //  Estimated: `0`
    // Minimum execution time: 120_000_000 picoseconds.
    Weight::from_parts(186_224_999_944, 0)
      // Standard Error: 2_501
      .saturating_add(Weight::from_parts(108_802, 0).saturating_mul(n.into()))
      .saturating_add(T::DbWeight::get().reads(4))
      .saturating_add(T::DbWeight::get().writes(2))
  }
  /// The range of component `a` is `[0, 3072]`.
  fn set_robonode_public_key(_a: u32, ) -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `0`
    //  Estimated: `0`
    // Minimum execution time: 3_000_000 picoseconds.
    Weight::from_parts(3_000_000, 0)
      .saturating_add(T::DbWeight::get().writes(2))
  }
  /// The range of component `a` is `[0, 3072]`.
  fn on_initialize(_a: u32, ) -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `141 + a * (40 ±0)`
    //  Estimated: `0`
    // Minimum execution time: 6_000_000 picoseconds.
    Weight::from_parts(96_000_000, 0)
      .saturating_add(T::DbWeight::get().reads(2))
      .saturating_add(T::DbWeight::get().writes(1))
  }
}
