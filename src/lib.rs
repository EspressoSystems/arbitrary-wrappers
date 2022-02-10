#![deny(warnings)]

use arbitrary::{Arbitrary, Unstructured};
use ark_std::UniformRand;
use jf_cap::keys::{UserAddress, UserKeyPair};
use jf_cap::structs::{FreezeFlag, Nullifier, ReceiverMemo, RecordOpening};
use jf_cap::{BaseField, KeyPair, MerkleTree};
use rand_chacha::{rand_core::SeedableRng, ChaChaRng};
use serde::{Deserialize, Serialize};
use zerok_macros::ser_test;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArbitraryNullifier(Nullifier);

impl From<Nullifier> for ArbitraryNullifier {
    fn from(n: Nullifier) -> Self {
        Self(n)
    }
}

impl From<ArbitraryNullifier> for Nullifier {
    fn from(n: ArbitraryNullifier) -> Self {
        n.0
    }
}

impl<'a> Arbitrary<'a> for ArbitraryNullifier {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut rng = ChaChaRng::from_seed(u.arbitrary()?);
        Ok(Self(Nullifier::random_for_test(&mut rng)))
    }
}

pub struct ArbitraryBaseField(pub BaseField);

impl From<ArbitraryBaseField> for BaseField {
    fn from(n: ArbitraryBaseField) -> Self {
        n.0
    }
}

impl<'a> Arbitrary<'a> for ArbitraryBaseField {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut rng = ChaChaRng::from_seed(u.arbitrary()?);
        Ok(Self(BaseField::rand(&mut rng)))
    }
}

pub struct ArbitraryRecordOpening(RecordOpening);

impl From<ArbitraryRecordOpening> for RecordOpening {
    fn from(ro: ArbitraryRecordOpening) -> Self {
        ro.0
    }
}

impl<'a> Arbitrary<'a> for ArbitraryRecordOpening {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut rng = ChaChaRng::from_seed(u.arbitrary()?);
        Ok(Self(
            RecordOpening::dummy(
                &mut rng,
                *u.choose(&[FreezeFlag::Frozen, FreezeFlag::Unfrozen])?,
            )
            .0,
        ))
    }
}

pub struct ArbitraryReceiverMemo(ReceiverMemo);

impl From<ArbitraryReceiverMemo> for ReceiverMemo {
    fn from(m: ArbitraryReceiverMemo) -> Self {
        m.0
    }
}

impl<'a> Arbitrary<'a> for ArbitraryReceiverMemo {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut rng = ChaChaRng::from_seed(u.arbitrary()?);
        Ok(Self(
            ReceiverMemo::from_ro(
                &mut rng,
                &u.arbitrary::<ArbitraryRecordOpening>()?.into(),
                &[],
            )
            .unwrap(),
        ))
    }
}

pub struct ArbitraryUserKeyPair(UserKeyPair);

impl From<ArbitraryUserKeyPair> for UserKeyPair {
    fn from(k: ArbitraryUserKeyPair) -> Self {
        k.0
    }
}

impl<'a> Arbitrary<'a> for ArbitraryUserKeyPair {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut rng = ChaChaRng::from_seed(u.arbitrary()?);
        Ok(Self(UserKeyPair::generate(&mut rng)))
    }
}

pub struct ArbitraryUserAddress(UserAddress);

impl From<ArbitraryUserAddress> for UserAddress {
    fn from(a: ArbitraryUserAddress) -> Self {
        a.0
    }
}

impl<'a> Arbitrary<'a> for ArbitraryUserAddress {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self(
            UserKeyPair::from(u.arbitrary::<ArbitraryUserKeyPair>()?).address(),
        ))
    }
}

pub struct ArbitraryKeyPair(KeyPair);

impl From<ArbitraryKeyPair> for KeyPair {
    fn from(k: ArbitraryKeyPair) -> Self {
        k.0
    }
}

impl<'a> Arbitrary<'a> for ArbitraryKeyPair {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut rng = ChaChaRng::from_seed(u.arbitrary()?);
        Ok(Self(KeyPair::generate(&mut rng)))
    }
}

#[ser_test(arbitrary, ark(false))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ArbitraryMerkleTree(pub MerkleTree);

impl<'a> Arbitrary<'a> for ArbitraryMerkleTree {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut mt = MerkleTree::new(3).unwrap();
        for _ in 0..15 {
            // todo: range restricted random depth and count
            mt.push(u.arbitrary::<ArbitraryBaseField>()?.into());
        }
        Ok(ArbitraryMerkleTree(mt))
    }
}
