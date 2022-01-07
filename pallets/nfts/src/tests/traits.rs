use super::mock::*;
use crate::tests::mock;
use crate::traits::{LockableNFTs, NFTs as NFTTrait};
use crate::Error;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

#[test]
fn lock_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path
            let nft_id = help::create(alice.clone(), vec![1], None);
            assert_ok!(NFTs::lock(nft_id));
            assert_eq!(NFTs::data(nft_id).unwrap().locked, true);
        })
}

#[test]
fn lock_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Unhappy already locked
            let nft_id = help::create(alice.clone(), vec![1], None);
            assert_ok!(NFTs::lock(nft_id));
            assert_noop!(NFTs::lock(nft_id), Error::<Test>::Locked);

            // Unhappy invalid NFT Id
            assert_noop!(NFTs::lock(1001), Error::<Test>::InvalidNFTId);
        })
}

#[test]
fn unlock_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path
            let nft_id = help::create(alice.clone(), vec![1], None);
            assert_ok!(NFTs::lock(nft_id));
            assert_eq!(NFTs::unlock(nft_id), true);
            assert_eq!(NFTs::data(nft_id).unwrap().locked, false);

            // Happy double unlock
            assert_eq!(NFTs::unlock(nft_id), true);
        })
}

#[test]
fn unlock_unhappy() {
    ExtBuilder::default().build().execute_with(|| {
        // Unhappy invalid NFT Id
        assert_eq!(NFTs::unlock(1001), false);
    })
}

#[test]
fn locked_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path
            let nft_id = help::create(alice.clone(), vec![1], None);
            assert_eq!(NFTs::locked(nft_id), Some(false));
            assert_ok!(NFTs::lock(nft_id));
            assert_eq!(NFTs::locked(nft_id), Some(true));
        })
}

#[test]
fn locked_unhappy() {
    ExtBuilder::default().build().execute_with(|| {
        // Unhappy invalid NFT Id
        assert_eq!(NFTs::locked(1001), None);
    })
}

#[test]
fn set_owner_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path
            let nft_id = help::create(alice.clone(), vec![1], None);
            assert_ok!(NFTs::set_owner(nft_id, &BOB));
            assert_eq!(NFTs::data(nft_id).unwrap().owner, BOB);
        })
}

#[test]
fn set_owner_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Unhappy is locked
            let nft_id = help::create(alice.clone(), vec![1], None);
            assert_ok!(NFTs::lock(nft_id));
            assert_noop!(NFTs::set_owner(nft_id, &BOB), Error::<Test>::Locked);

            // Unhappy invalid NFT Id
            assert_noop!(NFTs::set_owner(1000, &BOB), Error::<Test>::InvalidNFTId);
        })
}

#[test]
fn owner_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path
            let nft_id = help::create(alice.clone(), vec![1], None);
            assert_eq!(NFTs::owner(nft_id), Some(ALICE));
        })
}

#[test]
fn owner_unhappy() {
    ExtBuilder::default().build().execute_with(|| {
        // Unhappy invalid NFT Id
        assert_eq!(NFTs::owner(1000), None);
    })
}

#[test]
fn is_series_completed_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path
            let series_id = vec![50];
            let nft_id = help::create(alice.clone(), vec![1], Some(series_id.clone()));
            assert_eq!(NFTs::is_series_completed(nft_id), Some(false));
            assert_ok!(NFTs::finish_series(alice, series_id));
            assert_eq!(NFTs::is_series_completed(nft_id), Some(true));
        })
}

#[test]
fn is_series_completed_unhappy() {
    ExtBuilder::default().build().execute_with(|| {
        // Unhappy invalid NFT Id
        assert_eq!(NFTs::is_series_completed(1001), None);
    })
}