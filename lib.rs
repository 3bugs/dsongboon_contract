//cargo +nightly contract build
//https://stackoverflow.com/questions/63459771/how-do-i-save-a-string-value-on-substrates-smart-contract-platform-ink/67065138#67065138

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod dsongboon_contract {
    use scale::{Encode, Decode};
    use scale_info::TypeInfo;
    use ink_storage::collections::HashMap;
    use ink_prelude::string::String;
    use ink_storage::traits::{
        SpreadLayout,
        PackedLayout,
    };

    #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout, TypeInfo)]
    pub struct Songboon {
        // Songboon ID
        pub id: u32,
        pub recipient_name: String,
        pub certifier_name: String,
        pub bank_account_number: String,
        pub bank_account_name: String,
        pub bank_name: String,
        pub request_amount: u32,
        pub request_date: String,
        pub end_date: String,
        pub category: String,
        // keep files on file system, only store their hashes on chain
        pub request_document_file_hash: String,
        pub certificate_file_hash: String,
    }

    #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout, TypeInfo)]
    pub struct Tumboon {
        // Tumboon ID
        pub id: u32,
        pub songboon_id: u32,
        pub donor_name: String,
        // donor_type :- 0: individual, 1: organization
        pub donor_type: u8,
        pub donate_amount: u32,
        pub donate_date: String,
        pub slip_file_hash: String,
    }

    #[ink(storage)]
    pub struct DsongboonContract {
        str_value: String,
        value: i32,
        my_value: HashMap<AccountId, i32>,
        songboon_list: Vec<Songboon>,
        tumboon_list: Vec<Tumboon>,
    }

    impl DsongboonContract {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                str_value: String::from("abc"),
                value: init_value,
                my_value: HashMap::new(),
                songboon_list: Vec::new(),
                tumboon_list: Vec::new(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                str_value: String::from("def"),
                value: 0,
                my_value: Default::default(),
                songboon_list: Vec::new(),
                tumboon_list: Vec::new(),
            }
        }

        #[ink(message)]
        pub fn get_str(&self) -> String {
            self.str_value.clone()
        }

        #[ink(message)]
        pub fn set_str(&mut self, new_str: String) {
            self.str_value = new_str;
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            self.my_value_or_zero(&self.env().caller())
        }

        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            // ACTION: Get the `caller` of this function.
            let caller = self.env().caller();
            // ACTION: Get `my_value` that belongs to `caller` by using `my_value_or_zero`.
            let my_value = self.my_value_or_zero(&caller);
            // ACTION: Insert the incremented `value` back into the mapping.
            self.my_value.insert(caller, my_value + by);
        }

        fn my_value_or_zero(&self, of: &AccountId) -> i32 {
            *self.my_value.get(of).unwrap_or(&0)
        }

        // ดึงซองทั้งหมด
        #[ink(message)]
        pub fn get_songboon_list(&self) -> Vec<Songboon> {
            self.songboon_list.clone()
        }

        // เพิ่มซอง
        #[ink(message)]
        pub fn add_songboon(&mut self, songboon: Songboon) {
            self.songboon_list.push(songboon);
        }

        // ดึงการบริจาคทั้งหมด
        #[ink(message)]
        pub fn get_tumboon_list(&self) -> Vec<Tumboon> {
            self.tumboon_list.clone()
        }

        // ดึงการบริจาคทั้งหมดของซองหนึ่งๆ
        #[ink(message)]
        pub fn get_tumboon_list_by_songboon_id(&self, songboon_id: u32) -> Vec<Tumboon> {
            self.get_tumboon_list().into_iter().filter(|tumboon| tumboon.songboon_id == songboon_id).collect()
        }

        // เพิ่มการบริจาค
        #[ink(message)]
        pub fn add_tumboon(&mut self, tumboon: Tumboon) {
            self.tumboon_list.push(tumboon);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // Alias `ink_lang` so we can use `ink::test`.
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let contract = DsongboonContract::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = DsongboonContract::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }

        // Use `ink::test` to initialize accounts.
        #[ink::test]
        fn my_value_works() {
            let mut contract = DsongboonContract::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.inc_mine(10);
            assert_eq!(contract.get_mine(), 15);
        }

        #[ink::test]
        fn songboon_works() {
            let mut contract = DsongboonContract::new(0);
            assert_eq!(contract.get_songboon_list().len(), 0);

            contract.add_songboon(Songboon {
                id: 1,
                recipient_name: "สมบุญ ณ แองเจิ้ลไทม์".to_string(),
                certifier_name: "มูลนิธิกระจกเงา".to_string(),
                bank_account_number: "1234567890".to_string(),
                bank_account_name: "สมบุญ ณ แองเจิ้ลไทม์".to_string(),
                bank_name: "กสิกรไทย".to_string(),
                request_amount: 10000,
                request_date: "2020-01-14 13:00:00".to_string(),
                end_date: "2020-02-28 23:59:59".to_string(),
                category: "children".to_string(),
                request_document_file_hash: "e0d123e5f316bef78bfdf5a008837577".to_string(),
                certificate_file_hash: "35d91262b3c3ec8841b54169588c97f7".to_string(),
            });
            assert_eq!(contract.get_songboon_list().len(), 1);
        }

        #[ink::test]
        fn tumboon_works() {
            let mut contract = DsongboonContract::new(0);
            assert_eq!(contract.get_tumboon_list().len(), 0);

            contract.add_tumboon(Tumboon {
                id: 1,
                songboon_id: 1,
                donor_name: "เจริญพร ณ แองเจิ้ลไทม์".to_string(),
                donor_type: 0, // 0: individual, 1: organization
                donate_amount: 500,
                donate_date: "2020-01-14 16:30:00".to_string(),
                slip_file_hash: "2d86c4246f3c0eb516628bf324d6b9a3".to_string(),
            });
            assert_eq!(contract.get_tumboon_list().len(), 1);
            assert_eq!(contract.get_tumboon_list_by_songboon_id(1).len(), 1);

            contract.add_tumboon(Tumboon {
                id: 2,
                songboon_id: 1,
                donor_name: "เจริญบุญ ณ แองเจิ้ลไทม์".to_string(),
                donor_type: 0, // 0: individual, 1: organization
                donate_amount: 999,
                donate_date: "2020-01-14 19:45:00".to_string(),
                slip_file_hash: "66bd00e43ff8b932c14140472c4b8cc6".to_string(),
            });
            assert_eq!(contract.get_tumboon_list().len(), 2);
            assert_eq!(contract.get_tumboon_list_by_songboon_id(1).len(), 2);

            contract.add_tumboon(Tumboon {
                id: 3,
                songboon_id: 2,
                donor_name: "เจริญศรี ณ แองเจิ้ลไทม์".to_string(),
                donor_type: 0, // 0: individual, 1: organization
                donate_amount: 555,
                donate_date: "2020-01-15 06:00:00".to_string(),
                slip_file_hash: "14140472c4b8cc666bd00e43ff8b932c".to_string(),
            });
            assert_eq!(contract.get_tumboon_list().len(), 3);
            assert_eq!(contract.get_tumboon_list_by_songboon_id(1).len(), 2);
        }
    }
}
