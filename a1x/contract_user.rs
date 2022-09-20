#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod contract {
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
  pub struct User {
    pub id: u32,
    pub user_type: String,
    pub is_verify: bool,
    pub is_active: bool,
    pub method: String,
  }

  /// Defines the storage of your contract.
  /// Add new fields to the below struct in order
  /// to add new static storage fields to your contract.
  #[ink(storage)]
  #[derive(SpreadAllocate)]
  pub struct DsongboonUsers {
    user_map: Mapping<u32, User>,
    user_id_list: Vec<u32>,
  }

  impl DsongboonUsers {
    #[ink(constructor)]
    pub fn default() -> Self {
      ink_lang::utils::initialize_contract(|contract| {
        Self::init(contract)
      })
    }

    fn init(&mut self) {
    }

    #[ink(message)]
    pub fn add_user(
      &mut self,
      user: User,
    ) {
      self.user_map.insert(user.id, &user);
      self.user_id_list.push(user.id);
    }

    #[ink(message)]
    pub fn user_count(&self) -> u32 {
      self.user_id_list.len() as u32
    }

    #[ink(message)]
    pub fn get_user(&self, user_id: u32) -> User {
      self.user_map.get(&user_id).unwrap()
    }

    #[ink(message)]
    pub fn get_user_list(&self, from: u32, count: u32) -> Vec<User> {
      let mut user_list: Vec<User> = Vec::new();

      let list_length = self.user_id_list.len() as u32;
      let mut index = from;
      let mut count = if count > list_length {
        list_length
      } else {
        count
      };
      while index < list_length && count > 0 {
        let user = self.user_map.get(&self.user_id_list[index as usize]).unwrap();
        user_list.push(user);
        index += 1;
        count -= 1;
      }

      user_list
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
      let contract = DsongboonUsers::default();
      assert_eq!(contract.user_count(), 0);
    }

    #[ink::test]
    fn add_user_works() {
      let mut contract = DsongboonUsers::default();
      assert_eq!(contract.user_count(), 0);
      contract.add_user(User {
        id: 1,
        user_type: "USER".to_string(),
        is_verify: false,
        is_active: true,
        method: "register".to_string(),
      });
      assert_eq!(contract.user_count(), 1);
    }

    #[ink::test]
    fn get_user_works() {
      let mut contract = DsongboonUsers::default();
      contract.add_user(User {
        id: 111,
        user_type: "USER".to_string(),
        is_verify: false,
        is_active: true,
        method: "register".to_string(),
      });
      let user = contract.get_user(111);
      assert_eq!(user.id, 111);
    }
  }
}
