use casper_types::{
    testing::TestRng, Transaction, TransactionEntryPoint, TransactionInvocationTarget,
    TransactionScheduling,
};

// This exists only to warn us with compiler errors when one of the
// enum variants in the main node repo changes, for example when a new
// native entry point is added and should be handled.
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn pin() {
    let mut rng = TestRng::new();

    match Transaction::random(&mut rng) {
        Transaction::Deploy(_) => {}
        Transaction::V1(_) => {}
    }

    match TransactionEntryPoint::random(&mut rng) {
        TransactionEntryPoint::Call => {}
        TransactionEntryPoint::Custom(_) => {}
        TransactionEntryPoint::Transfer => {}
        TransactionEntryPoint::Burn => {}
        TransactionEntryPoint::AddBid => {}
        TransactionEntryPoint::WithdrawBid => {}
        TransactionEntryPoint::Delegate => {}
        TransactionEntryPoint::Undelegate => {}
        TransactionEntryPoint::Redelegate => {}
        TransactionEntryPoint::ActivateBid => {}
        TransactionEntryPoint::ChangeBidPublicKey => {}
        TransactionEntryPoint::AddReservations => {}
        TransactionEntryPoint::CancelReservations => {}
    };

    match TransactionScheduling::random(&mut rng) {
        TransactionScheduling::Standard => {}
    }

    match TransactionInvocationTarget::random(&mut rng) {
        TransactionInvocationTarget::ByHash(_) => {}
        TransactionInvocationTarget::ByName(_) => {}
        TransactionInvocationTarget::ByPackageHash { addr, version } => {}
        TransactionInvocationTarget::ByPackageName { name, version } => {}
    }
}
