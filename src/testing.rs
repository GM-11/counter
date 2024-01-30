#[cfg(test)]
mod test {
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    use std::mem;
    use std::vec;

    use borsh::BorshDeserialize;
    use solana_program::account_info::AccountInfo;

    use crate::{process_instruction, CounterAccount};

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();

        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];

        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        let mut increment_inx_data: Vec<u8> = vec![0];
        let mut decrement_inx_data: Vec<u8> = vec![1];
        let mut update_inx_data: Vec<u8> = vec![2];
        let reset_inx_data: Vec<u8> = vec![3];

        let incr_value: u32 = 21;
        increment_inx_data.extend_from_slice(&incr_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &increment_inx_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            21
        );

        let decr_value: u32 = 10;
        decrement_inx_data.extend_from_slice(&decr_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &decrement_inx_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            11
        );

        let update_value: u32 = 33;
        update_inx_data.extend_from_slice(&update_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &update_inx_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            33
        );

        process_instruction(&program_id, &accounts, &reset_inx_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
