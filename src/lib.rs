#![deny(warnings)]

use arbitrary::{Arbitrary, Unstructured};
use ark_std::UniformRand;
use jf_aap::keys::{UserAddress, UserKeyPair};
use jf_aap::structs::{FreezeFlag, Nullifier, ReceiverMemo, RecordOpening};
use jf_aap::{BaseField, KeyPair};
use rand_chacha::{rand_core::SeedableRng, ChaChaRng};

#[derive(PartialEq, Eq, Hash)]
pub struct ArbitraryNullifier(Nullifier);

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
