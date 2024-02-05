#![cfg(feature = "test-sbf")]

mod program_test;

use {program_test::*, solana_program_test::tokio};

// TODO: Test only authority which set the lock can remove it
// test: Remove lock for authority no longer on list of accepted authorities
// test: Try to remove for other authority
// test: Try to remove without signing

#[tokio::test]
async fn test_remove_token_owner_record_lock() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;

    let token_owner_record_cookie = governance_test
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    let token_owner_record_lock_authority_cookie = governance_test
        .with_community_token_owner_record_lock_authority(&realm_cookie)
        .await
        .unwrap();

    let token_owner_record_lock_cookie = governance_test
        .with_token_owner_record_lock(
            &token_owner_record_cookie,
            &token_owner_record_lock_authority_cookie,
        )
        .await
        .unwrap();

    // Act
    governance_test
        .remove_token_owner_record_lock(
            &token_owner_record_cookie,
            &token_owner_record_lock_authority_cookie.authority,
            token_owner_record_lock_cookie.lock_type,
        )
        .await
        .unwrap();

    // Assert
    let token_owner_record_account = governance_test
        .get_token_owner_record_account(&token_owner_record_cookie.address)
        .await;

    assert_eq!(0, token_owner_record_account.locks.len());
}
