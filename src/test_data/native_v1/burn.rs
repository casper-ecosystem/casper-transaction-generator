use casper_types::{
    runtime_args, AccessRights, RuntimeArgs, TransactionArgs, TransactionEntryPoint,
    TransactionScheduling, TransactionTarget, URef, U512,
};

use crate::sample::Sample;

use crate::test_data::TransactionV1Meta;

/// Represents native 'burn' sample.
#[derive(Clone, Debug)]
struct Burn {
    source: URef,
    amount: U512,
}

impl Burn {
    pub fn new(source: URef, amount: U512) -> Self {
        Self { source, amount }
    }
}

impl From<Burn> for RuntimeArgs {
    fn from(d: Burn) -> Self {
        let mut args = RuntimeArgs::new();
        args.insert("source", d.source).unwrap();
        args.insert("amount", d.amount).unwrap();
        args
    }
}

// Generate a native burn sample for every possible combination of parameters
fn native_burn_samples(source_urefs: &[URef], burn_amounts: &[U512]) -> Vec<Sample<Burn>> {
    let mut samples: Vec<Sample<Burn>> = vec![];

    for source in source_urefs {
        for amount in burn_amounts {
            let burn = Burn::new(*source, *amount);

            samples.push(Sample::new("native_burn_v1", burn, true));
        }
    }

    samples
}

/// Returns valid burn samples.
pub(crate) fn valid() -> Vec<Sample<TransactionV1Meta>> {
    let sources = vec![
        URef::new([1; 32], AccessRights::NONE),
        URef::new([2; 32], AccessRights::READ),
        URef::new([3; 32], AccessRights::WRITE),
        URef::new([4; 32], AccessRights::ADD),
        URef::new([5; 32], AccessRights::READ_ADD),
        URef::new([6; 32], AccessRights::READ_WRITE),
        URef::new([7; 32], AccessRights::ADD_WRITE),
        URef::new([8; 32], AccessRights::READ_ADD_WRITE),
    ];

    let amounts = vec![U512::from(0), U512::from(1_000_000), U512::MAX];

    super::make_samples_with_schedulings(
        native_burn_samples(&sources, &amounts),
        TransactionEntryPoint::Burn,
    )
}

/// Returns invalid burn samples.
pub(crate) fn invalid() -> Vec<Sample<TransactionV1Meta>> {
    let missing_source = runtime_args! {
        "amount" => U512::from(1_000),
    };

    let missing_amount = runtime_args! {
        "source" => URef::new([8; 32], AccessRights::READ_ADD_WRITE),
    };

    let invalid_args = vec![
        Sample::new("missing_source", missing_source, false),
        Sample::new("missing_amount", missing_amount, false),
    ];

    invalid_args
        .into_iter()
        .map(|sample_ra| {
            let (label, ra, validity) = sample_ra.destructure();
            let sample_invalid_burn = TransactionV1Meta::new(
                TransactionArgs::Named(ra),
                TransactionTarget::Native,
                TransactionEntryPoint::Burn,
                TransactionScheduling::Standard,
            );
            let new_label = format!("native_burn_v1_{}", label);
            Sample::new(new_label, sample_invalid_burn, validity)
        })
        .collect()
}
