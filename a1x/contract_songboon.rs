#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod dsongboon_songboon {
  use scale::{Encode, Decode};
  use ink_prelude::vec::Vec;
  use ink_prelude::string::String;
  use ink_storage::Mapping;
  use ink_storage::traits::{
    SpreadLayout,
    PackedLayout,
    SpreadAllocate,
  };

  #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout)]
  #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
  pub struct Tumboon {
    pub donor_wallet_address: String,
    pub donate_amount: u32,
    pub donate_date: String,
    pub currency_unit: String,
  }

  #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout)]
  #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
  pub struct PartialSongboon {
    pub id: u32,
    pub requester_wallet_address: String,
    pub requester: u32,
    pub user_type: String,
    pub post_name: String,
    pub request_amount: u32,
    pub currency_unit: String,
    pub expire_date: String,
  }

  /// Defines the storage of your contract.
  /// Add new fields to the below struct in order
  /// to add new static storage fields to your contract.
  #[ink(storage)]
  #[derive(SpreadAllocate)]
  pub struct DsongboonSongboon {
    id: u32,
    requester_wallet_address: String,
    requester: u32,
    user_type: String,
    post_name: String,
    request_amount: u32,
    currency_unit: String,
    expire_date: String,
    // List of Tumboon in this Songboon instance
    tumboon_map: Mapping<u32, Tumboon>,
    tumboon_id_list: Vec<u32>,
  }

  impl DsongboonSongboon {
    #[ink(constructor)]
    pub fn default(
      id: u32,
      requester_wallet_address: String,
      requester: u32,
      user_type: String,
      post_name: String,
      request_amount: u32,
      currency_unit: String,
      expire_date: String,
    ) -> Self {
      ink_lang::utils::initialize_contract(|contract| {
        Self::init(contract, id, requester_wallet_address, requester, user_type, post_name, request_amount, currency_unit, expire_date)
      })
    }

    fn init(
      &mut self,
      id: u32,
      requester_wallet_address: String,
      requester: u32,
      user_type: String,
      post_name: String,
      request_amount: u32,
      currency_unit: String,
      expire_date: String,
    ) {
      self.id = id;
      self.requester_wallet_address = requester_wallet_address;
      self.requester = requester;
      self.user_type = user_type;
      self.post_name = post_name;
      self.request_amount = request_amount;
      self.currency_unit = currency_unit;
      self.expire_date = expire_date;
    }

    /*#[ink(message)]
    pub fn get_songboon(&self) -> PartialSongboon {
      PartialSongboon {
        id: self.id,
        requester_wallet_address: self.requester_wallet_address.clone(),
        requester: self.requester.clone(),
        user_type: self.user_type.clone(),
        post_name: self.post_name.clone(),
        request_amount: self.request_amount,
        currency_unit: self.currency_unit.clone(),
        expire_date: self.expire_date.clone(),
      }
    }*/

    #[ink(message)]
    pub fn get_id(&self) -> u32 {
      self.id
    }

    #[ink(message)]
    pub fn get_post_name(&self) -> String {
      self.post_name.clone()
    }

    #[ink(message)]
    pub fn get_request_amount(&self) -> u32 {
      self.request_amount
    }

    #[ink(message)]
    pub fn add_tumboon(
      &mut self,
      tumboon: Tumboon,
    ) {
      let id = self.tumboon_count();
      self.tumboon_map.insert(id, &tumboon);
      self.tumboon_id_list.push(id);
    }

    #[ink(message)]
    pub fn tumboon_count(&self) -> u32 {
      self.tumboon_id_list.len() as u32
    }

    #[ink(message)]
    pub fn get_total_tumboon_amount(&self) -> u32 {
      let list_length = self.tumboon_id_list.len() as u32;
      let mut index = 0;
      let mut total = 0;
      while index < list_length {
        let tumboon = self.tumboon_map.get(&self.tumboon_id_list[index as usize]).unwrap();
        total += tumboon.donate_amount;
        index += 1;
      }
      total
    }
  }

  /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
  /// module and test functions are marked with a `#[test]` attribute.
  /// The below code is technically just normal Rust code.
  #[cfg(test)]
  mod tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;

    /// Imports `ink_lang` so we can use `#[ink::test]`.
    use ink_lang as ink;

    #[ink::test]
    fn default_works() {
      /*
      id: u32,
      requester_wallet_address: String,
      requester: u32,
      user_type: String,
      post_name: String,
      request_amount: u32,
      currency_unit: String,
      expire_date: String,
      */
      let songboon = DsongboonSongboon::default(
        45,
        "0x1234567890".to_string(),
        30000,
        "user".to_string(),
        "ขอเชิญร่วมบริจาคสมทบทุน มูลนิธิธรรมรักษ์ วัดพระบาทน้ำพุ".to_string(),
        580000,
        "BHT".to_string(),
        "31-12-2022".to_string(),
      );
      assert_eq!(songboon.id, 45);
      assert_eq!(songboon.requester_wallet_address, "0x1234567890".to_string());
    }

    #[ink::test]
    fn add_tumboon_works() {
      let mut songboon = DsongboonSongboon::default(
        45,
        "0x1234567890".to_string(),
        30000,
        "user".to_string(),
        "ขอเชิญร่วมบริจาคสมทบทุน มูลนิธิธรรมรักษ์ วัดพระบาทน้ำพุ".to_string(),
        580000,
        "BHT".to_string(),
        "31-12-2022".to_string(),
      );
      assert_eq!(songboon.tumboon_count(), 0);
      songboon.add_tumboon(Tumboon {
        donor_wallet_address: "0x2345678901".to_string(),
        donate_amount: 111,
        donate_date: "15-08-2022 01:30:00".to_string(),
        currency_unit: "BHT".to_string(),
      });
      assert_eq!(songboon.tumboon_count(), 1);
      assert_eq!(songboon.get_total_tumboon_amount(), 111);
      songboon.add_tumboon(Tumboon {
        donor_wallet_address: "0x2345678901".to_string(),
        donate_amount: 222,
        donate_date: "15-08-2022 01:30:00".to_string(),
        currency_unit: "BHT".to_string(),
      });
      assert_eq!(songboon.tumboon_count(), 2);
      assert_eq!(songboon.get_total_tumboon_amount(), 333);
    }
  }
}
