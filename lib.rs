//cargo +nightly contract build
//https://stackoverflow.com/questions/63459771/how-do-i-save-a-string-value-on-substrates-smart-contract-platform-ink/67065138#67065138

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod dsongboon {
  use scale::{Encode, Decode};
  //use ink_storage::collections::Vec;
  use ink_prelude::vec::Vec;
  use ink_prelude::string::String;
  use ink_storage::Mapping;
  use ink_storage::traits::{
    SpreadLayout,
    PackedLayout,
    SpreadAllocate,
  };

  // https://github.com/paritytech/cargo-contract/issues/158

  #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout)]
  #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
  pub struct Songboon {
    // Songboon ID
    pub id: u32,
    pub donate_req_number: String,
    pub donate_req_topic: String,
    pub donate_req_detail: String,
    pub donate_req_hashtag: Option<String>,
    pub donate_req_by: u32,
    pub donate_req_by_name: String,
    pub donate_req_by_id_card_number: String,
    pub address: String,
    pub province: String,
    pub is_organization: bool,
    pub organization: Option<u32>,
    pub organization_name: Option<String>,
    pub donate_category: String,
    pub donate_req_date: String,
    pub donate_doc_expire_time: String,
    pub donate_doc_status: String,
    pub total_req_amount: u32,
    pub account_bank: Option<String>,
    pub account_number: Option<String>,
    pub account_name: String,
    pub account_promptpay_nid: Option<String>,
    pub account_promptpay_phone: Option<String>,
    pub certificates: [Option<Certificate>; 3],
  }

  #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout)]
  #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
  pub struct Tumboon {
    // Tumboon ID
    pub id: u32,
    pub songboon_id: u32,
    pub move_amount: u32,
    pub move_date: String,
    pub donate_by: u32,
    pub donate_by_name: String,
    pub donate_by_info_display: u32,
    pub slip_file_url: Option<String>,
    pub slip_file_hash: Option<String>,
    pub slip_pay_amount: u32,
    pub slip_pay_ref: String,
    pub slip_pay_time: String,
    pub slip_pay_from_bank: String,
    pub slip_pay_from_account: String,
    pub slip_pay_from_name: String,
    pub slip_pay_to_bank: Option<String>,
    pub slip_pay_to_account: String,
    pub slip_pay_to_name: String,
    pub total_req_amount_history_before: u32,
    pub total_receive_amount_history_before: u32,
    pub total_remain_amount_history_before: u32,
    pub total_req_amount_history_after: u32,
    pub total_receive_amount_history_after: u32,
    pub total_remain_amount_history_after: i32,
  }

  #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout)]
  #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
  pub struct Certificate {
    pub certificate_position: String,
    pub certificate_date: String,
    // เอกสารรับรอง
    pub file_url: String,
    pub file_hash: String,
    // ไฟล์ภาพลายเซ็น
    pub signature_url: String,
    pub signature_hash: String,
  }

  #[derive(Debug, Clone, Encode, Decode)]
  #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
  pub struct Summary {
    pub total_req_amount: u32,
    pub total_donate_amount: u32,
    pub total_req_amount_current_user: u32,
    pub total_donate_amount_current_user: u32,
    pub songboon_count: u32,
    pub tumboon_count: u32,
    pub songboon_count_current_user: u32,
    pub tumboon_count_current_user: u32,
  }

  #[ink(storage)]
  #[derive(SpreadAllocate)]
  pub struct Dsongboon {
    songboon_map: Mapping<u32, Songboon>,
    songboon_id_list: Vec<u32>,
    tumboon_map: Mapping<u32, Tumboon>,
    tumboon_id_list: Vec<u32>,
  }

  impl Dsongboon {
    #[ink(constructor)]
    pub fn default() -> Self {
      ink_lang::utils::initialize_contract(|_| {})
    }

    //--------------------------------------------------------------------------
    //----- SONGBOON -----------------------------------------------------------
    //--------------------------------------------------------------------------

    #[ink(message)]
    pub fn add_songboon(&mut self, songboon: Songboon) {
      self.songboon_map.insert(songboon.id, &songboon);
      self.songboon_id_list.push(songboon.id);
    }

    #[ink(message)]
    pub fn get_songboon(&self, songboon_id: u32) -> Songboon {
      self.songboon_map.get(&songboon_id).unwrap()
    }

    // ถ้ามี Songboon ถึงจำนวนหนึ่ง จะเกิด ContractTrapped error
    /*#[ink(message)]
    pub fn get_all_songboon(&self) -> Vec<Songboon> {
      let mut songboon_list: Vec<Songboon> = Vec::new();

      for songboon_id in &self.songboon_id_list {
        let songboon = self.songboon_map.get(&songboon_id).unwrap();
        songboon_list.push(songboon);
        //println!("{:?}", songboon);
      }

      songboon_list
    }*/

    #[ink(message)]
    pub fn get_songboon_list(&self, from: u32, count: u32) -> Vec<Songboon> {
      let mut songboon_list: Vec<Songboon> = Vec::new();

      let list_length = self.songboon_id_list.len() as u32;
      let mut index = from;
      let mut count = if count > list_length {
        list_length
      } else {
        count
      };
      while index < list_length && count > 0 {
        let songboon = self.songboon_map.get(&self.songboon_id_list[index as usize]).unwrap();
        songboon_list.push(songboon);
        index += 1;
        count -= 1;
      }

      songboon_list
    }

    #[ink(message)]
    pub fn songboon_count(&self) -> u32 {
      self.songboon_id_list.len() as u32
    }

    #[ink(message)]
    pub fn add_certificate(&mut self, songboon_id: u32, certificate: Certificate) {
      let mut songboon = self.songboon_map.get(&songboon_id).unwrap();
      if songboon.certificates[0].is_none() {
        songboon.certificates[0] = Some(certificate);
        /*songboon.certificates[0] = Some(Certificate {
          certificate_position: certificate.certificate_position.clone(),
          certificate_date: certificate.certificate_date.clone(),
          file_url: certificate.file_url.clone(),
          file_hash: certificate.file_hash.clone(),
          signature_hash: certificate.signature_hash.clone(),
          signature_url: certificate.signature_url.clone(),
        });*/
      } else if songboon.certificates[1].is_none() {
        songboon.certificates[1] = Some(certificate);
      } else if songboon.certificates[2].is_none() {
        songboon.certificates[2] = Some(certificate);
      }
      self.songboon_map.insert(songboon_id, &songboon);
    }

    #[ink(message)]
    pub fn update_songboon_status(&mut self, songboon_id: u32, status: String) {
      let mut songboon = self.songboon_map.get(&songboon_id).unwrap();
      songboon.donate_doc_status = status;
      self.songboon_map.insert(songboon_id, &songboon);
    }

    //--------------------------------------------------------------------------
    //----- TUMBOON ------------------------------------------------------------
    //--------------------------------------------------------------------------

    #[ink(message)]
    pub fn add_tumboon(&mut self, tumboon: Tumboon) {
      self.tumboon_map.insert(tumboon.id, &tumboon);
      self.tumboon_id_list.push(tumboon.id);
    }

    #[ink(message)]
    pub fn get_tumboon(&self, tumboon_id: u32) -> Tumboon {
      self.tumboon_map.get(&tumboon_id).unwrap()
    }

    // ถ้ามี Tumboon ถึงจำนวนหนึ่ง จะเกิด ContractTrapped error
    /*#[ink(message)]
    pub fn get_all_tumboon(&self) -> Vec<Tumboon> {
      let mut tumboon_list: Vec<Tumboon> = Vec::new();

      for tumboon_id in &self.tumboon_id_list {
        let tumboon = self.tumboon_map.get(&tumboon_id).unwrap();
        tumboon_list.push(tumboon);
        //println!("{:?}", tumboon);
      }

      tumboon_list
    }*/

    #[ink(message)]
    pub fn get_tumboon_list(&self, from: u32, count: u32) -> Vec<Tumboon> {
      let mut tumboon_list: Vec<Tumboon> = Vec::new();

      let list_length = self.tumboon_id_list.len() as u32;
      let mut index = from;
      let mut count = if count > list_length {
        list_length
      } else {
        count
      };
      while index < list_length && count > 0 {
        let tumboon = self.tumboon_map.get(&self.tumboon_id_list[index as usize]).unwrap();
        tumboon_list.push(tumboon);
        index += 1;
        count -= 1;
      }

      tumboon_list
    }

    #[ink(message)]
    pub fn tumboon_count(&self) -> u32 {
      self.tumboon_id_list.len() as u32
    }

    //--------------------------------------------------------------------------
    //--------------------------------------------------------------------------
    //--------------------------------------------------------------------------

    #[ink(message)]
    pub fn get_summary(&self, user_id: u32) -> Summary {
      let mut summary = Summary {
        total_req_amount: 0,
        total_donate_amount: 0,
        total_req_amount_current_user: 0,
        total_donate_amount_current_user: 0,
        songboon_count: 0,
        tumboon_count: 0,
        songboon_count_current_user: 0,
        tumboon_count_current_user: 0,
      };
      //let mut summary: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

      summary.songboon_count = self.songboon_count();
      summary.tumboon_count = self.tumboon_count();

      for songboon_id in &self.songboon_id_list {
        let songboon = self.songboon_map.get(&songboon_id).unwrap();
        summary.total_req_amount += songboon.total_req_amount;

        if user_id > 0 && songboon.donate_req_by == user_id {
          summary.total_req_amount_current_user += songboon.total_req_amount;
          summary.songboon_count_current_user += 1;
        }
      }

      for tumboon_id in &self.tumboon_id_list {
        let tumboon = self.tumboon_map.get(&tumboon_id).unwrap();
        summary.total_donate_amount += tumboon.move_amount;
        if user_id > 0 && tumboon.donate_by == user_id {
          summary.total_donate_amount_current_user += tumboon.move_amount;
          summary.tumboon_count_current_user += 1;
        }
      }

      summary
    }

    /*#[ink(message)]
    pub fn get_summary(&self, user_id: u32) -> [u32; 4] {
      let mut summary: [u32; 4] = [0, 0, 0, 0];

      self.get_all_songboon().iter().for_each(|songboon| {
        summary[0] += songboon.total_req_amount;
        if songboon.donate_req_by == user_id {
          summary[2] += songboon.total_req_amount;
        }
      });
      self.get_all_tumboon().iter().for_each(|tumboon| {
        summary[1] += tumboon.move_amount;
        if tumboon.donate_by == user_id {
          summary[3] += tumboon.move_amount;
        }
      });

      summary
    }*/
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    // Alias `ink_lang` so we can use `ink::test`.
    use ink_lang as ink;

    #[ink::test]
    fn songboon_works() {
      let mut contract = Dsongboon::default();
      //assert_eq!(contract.get_songboon_count(), 0);
      let songboon = Songboon {
        id: 1,
        donate_req_number: "".to_string(),
        donate_req_topic: "".to_string(),
        donate_req_detail: "".to_string(),
        donate_req_hashtag: None,
        donate_req_by: 0,
        donate_req_by_name: "".to_string(),
        donate_req_by_id_card_number: "".to_string(),
        address: "".to_string(),
        province: "".to_string(),
        is_organization: false,
        organization: None,
        organization_name: None,
        donate_category: "".to_string(),
        donate_req_date: "".to_string(),
        donate_doc_expire_time: "".to_string(),
        donate_doc_status: "".to_string(),
        total_req_amount: 10000,
        account_bank: None,
        account_number: None,
        account_name: "".to_string(),
        account_promptpay_nid: None,
        account_promptpay_phone: None,
        certificates: [
          Some(Certificate {
            certificate_position: "xxx".to_string(),
            certificate_date: "xxx".to_string(),
            file_url: "xxx".to_string(),
            file_hash: "xxx".to_string(),
            signature_url: "xxx".to_string(),
            signature_hash: "xxx".to_string(),
          }),
          None,
          None,
        ],
      };

      for i in 1..=100 {
        let mut sb = songboon.clone();
        sb.id = i;
        sb.total_req_amount = i * 1000;
        contract.add_songboon(sb);
        assert_eq!(contract.songboon_count(), i);
        assert_eq!(contract.get_songboon(i).id, i);
      }

      let list = contract.get_songboon_list(10, 5);
      let sum = list.iter().fold(0, |total, sb| {
        total + sb.total_req_amount
      });
      assert_eq!(sum, 11 * 1000 + 12 * 1000 + 13 * 1000 + 14 * 1000 + 15 * 1000);
      assert_eq!(contract.get_songboon_list(10, 5).len(), 5);
      assert_eq!(contract.get_songboon_list(10, 1000).len(), 90);

      /*assert_eq!(contract.get_all_songboon().len(), 2);
      let mut songboon = contract.get_all_songboon()[0].clone();
      assert_eq!(songboon.id, 1);
      songboon = contract.get_all_songboon()[1].clone();
      assert_eq!(songboon.id, 2);*/

      /*assert_eq!(contract.songboon_list_count(), 0);
      assert_eq!(contract.tumboon_list_count(), 0);*/
    }

    #[ink::test]
    fn add_certificate_works() {
      let mut contract = Dsongboon::default();
      contract.add_songboon(Songboon {
        id: 1,
        donate_req_number: "abc-123".to_string(),
        donate_req_topic: "ขอทุนการศึกษา".to_string(),
        donate_req_detail: "ขอทุนการศึกษา ขอทุนการศึกษา ขอทุนการศึกษา".to_string(),
        donate_req_hashtag: Option::from("#ทุนการศึกษา".to_string()),
        donate_req_by: 1,
        donate_req_by_name: "สมบุญ แองเจิ้ลไทม์".to_string(),
        donate_req_by_id_card_number: "1234567890123".to_string(),
        address: "11/13 ซ.งามวงศ์วาน 59 ลาดยาว".to_string(),
        province: "กรุงเทพมหานคร".to_string(),
        is_organization: false,
        organization: Option::from(0),
        organization_name: Option::from("มูลนิธิกระจกเงา".to_string()),
        donate_category: "children".to_string(),
        donate_req_date: "2020-01-14 13:00:00".to_string(),
        donate_doc_expire_time: "2020-02-28 23:59:59".to_string(),
        donate_doc_status: "OPE".to_string(),
        total_req_amount: 10000,
        account_bank: Option::from("ธนาคารกรุงเทพ".to_string()),
        account_number: Option::from("1234567890".to_string()),
        account_name: "สมบุญ แองเจิ้ลไทม์".to_string(),
        account_promptpay_nid: None,
        account_promptpay_phone: None,
        certificates: [
          None,
          None,
          None,
        ],
      });
      let songboon = contract.get_songboon(1);
      assert!(songboon.certificates[0].is_none());
      assert!(songboon.certificates[1].is_none());
      assert!(songboon.certificates[2].is_none());
      let certificate = Certificate {
        certificate_position: "certificate_position".to_string(),
        certificate_date: "certificate_date".to_string(),
        file_url: "certificate_doc_url".to_string(),
        file_hash: "certificate_doc_hash".to_string(),
        signature_url: "certificate_signature_url".to_string(),
        signature_hash: "certificate_signature_hash".to_string(),
      };
      contract.add_certificate(1, certificate);
      let songboon = contract.get_songboon(1);
      assert!(!songboon.certificates[0].is_none());
      assert!(songboon.certificates[1].is_none());
      assert!(songboon.certificates[2].is_none());
    }

    #[ink::test]
    fn update_songboon_status_works() {
      let mut contract = Dsongboon::default();
      contract.add_songboon(Songboon {
        id: 1,
        donate_req_number: "abc-123".to_string(),
        donate_req_topic: "ขอทุนการศึกษา".to_string(),
        donate_req_detail: "ขอทุนการศึกษา ขอทุนการศึกษา ขอทุนการศึกษา".to_string(),
        donate_req_hashtag: Option::from("#ทุนการศึกษา".to_string()),
        donate_req_by: 1,
        donate_req_by_name: "สมบุญ แองเจิ้ลไทม์".to_string(),
        donate_req_by_id_card_number: "1234567890123".to_string(),
        address: "11/13 ซ.งามวงศ์วาน 59 ลาดยาว".to_string(),
        province: "กรุงเทพมหานคร".to_string(),
        is_organization: false,
        organization: Option::from(0),
        organization_name: Option::from("มูลนิธิกระจกเงา".to_string()),
        donate_category: "children".to_string(),
        donate_req_date: "2020-01-14 13:00:00".to_string(),
        donate_doc_expire_time: "2020-02-28 23:59:59".to_string(),
        donate_doc_status: "OPE".to_string(),
        total_req_amount: 10000,
        account_bank: Option::from("ธนาคารกรุงเทพ".to_string()),
        account_number: Option::from("1234567890".to_string()),
        account_name: "สมบุญ แองเจิ้ลไทม์".to_string(),
        account_promptpay_nid: None,
        account_promptpay_phone: None,
        certificates: [
          None,
          None,
          None,
        ],
      });
      contract.update_songboon_status(1, "CLO".to_string());
      let songboon = contract.get_songboon(1);
      assert_eq!(songboon.donate_doc_status, "CLO".to_string());
    }
  }
}
