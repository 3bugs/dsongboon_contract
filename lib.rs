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
  use ink_storage::traits::{
    SpreadLayout,
    PackedLayout,
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
    pub slip_file_url: String,
    pub slip_file_hash: String,
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
    pub total_remain_amount_history_after: u32,
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

  #[ink(storage)]
  pub struct Dsongboon {
    songboon_list: Vec<Songboon>,
    tumboon_list: Vec<Tumboon>,
    vec: Vec<u32>,
  }

  impl Dsongboon {
    /*#[ink(constructor)]
    pub fn new(songboon_list: Vec<Songboon>, tumboon_list: Vec<Tumboon>) -> Self {
      Self {
        songboon_list,
        tumboon_list,
      }
    }*/

    #[ink(constructor)]
    pub fn default() -> Self {
      Self {
        songboon_list: Vec::new(),
        tumboon_list: Vec::new(),
        vec: Vec::new(),
      }
    }

    #[ink(message)]
    pub fn add_songboon(&mut self, songboon: Songboon) {
      self.songboon_list.push(songboon);
      self.vec.push(1);
    }

    #[ink(message)]
    pub fn add_vec(&mut self, value: u32) {
      self.vec.push(value);
    }

    #[ink(message)]
    pub fn get_vec(&self, index: u32) -> u32 {
      self.vec[index as usize]
    }

    #[ink(message)]
    pub fn vec_len(&self) -> u32 {
      self.vec.len() as u32
    }

    #[ink(message)]
    pub fn get_vec_list(&self) -> Vec<u32> {
      self.vec.clone()
    }

    //https://stackoverflow.com/questions/44662312/how-to-filter-a-vector-of-custom-structs-in-rust
    #[ink(message)]
    pub fn add_certificate(&mut self, songboon_id: u32, certificate: Certificate) -> bool {
      let mut added = false;
      self.songboon_list.iter_mut().for_each(|songboon| {
        if songboon.id == songboon_id {
          added = true;
          if songboon.certificates[0].is_none() {
            songboon.certificates[0] = Some(Certificate {
              certificate_position: certificate.certificate_position.clone(),
              certificate_date: certificate.certificate_date.clone(),
              file_url: certificate.file_url.clone(),
              file_hash: certificate.file_hash.clone(),
              signature_hash: certificate.signature_hash.clone(),
              signature_url: certificate.signature_url.clone(),
            });
          } else if songboon.certificates[1].is_none() {
            songboon.certificates[1] = Some(Certificate {
              certificate_position: certificate.certificate_position.clone(),
              certificate_date: certificate.certificate_date.clone(),
              file_url: certificate.file_url.clone(),
              file_hash: certificate.file_hash.clone(),
              signature_hash: certificate.signature_hash.clone(),
              signature_url: certificate.signature_url.clone(),
            });
          } else if songboon.certificates[2].is_none() {
            songboon.certificates[2] = Some(Certificate {
              certificate_position: certificate.certificate_position.clone(),
              certificate_date: certificate.certificate_date.clone(),
              file_url: certificate.file_url.clone(),
              file_hash: certificate.file_hash.clone(),
              signature_hash: certificate.signature_hash.clone(),
              signature_url: certificate.signature_url.clone(),
            });
          } else {
            added = false;
          }
        }
      });
      added
    }

    #[ink(message)]
    pub fn update_songboon_status(&mut self, songboon_id: u32, status: String) -> bool {
      let mut updated = false;
      self.songboon_list.iter_mut().for_each(|songboon| {
        if songboon.id == songboon_id {
          updated = true;
          songboon.donate_doc_status = status.clone();
        }
      });
      updated
    }

    // ดึงซองทั้งหมด
    #[ink(message)]
    pub fn get_songboon_list(&self) -> Vec<Songboon> {
      self.songboon_list.clone()
    }

    #[ink(message)]
    pub fn get_songboon(&self, id: u32) -> Option<Songboon> {
      self.songboon_list.get(id as usize).cloned()
    }

    #[ink(message)]
    pub fn get_songboon_by_id(&self, id: u32) -> Option<Songboon> {
      self.songboon_list.iter().find(|songboon| songboon.id == id).cloned()
    }

    #[ink(message)]
    pub fn songboon_list_count(&self) -> u32 {
      self.songboon_list.len() as u32
    }

    // เพิ่มการบริจาค
    #[ink(message)]
    pub fn add_tumboon(&mut self, tumboon: Tumboon) {
      self.tumboon_list.push(tumboon);
    }

    // ดึงการบริจาคทั้งหมด
    #[ink(message)]
    pub fn get_tumboon_list(&self) -> Vec<Tumboon> {
      self.tumboon_list.clone()
    }

    #[ink(message)]
    pub fn get_tumboon(&self, id: u32) -> Option<Tumboon> {
      self.tumboon_list.get(id as usize).cloned()
    }

    #[ink(message)]
    pub fn get_tumboon_by_id(&self, id: u32) -> Option<Tumboon> {
      self.tumboon_list.iter().find(|tumboon| tumboon.id == id).cloned()
    }

    #[ink(message)]
    pub fn get_tumboon_list_by_songboon_id(&self, songboon_id: u32) -> Vec<Tumboon> {
      let mut tumboon_list = Vec::new();
      for tumboon in &self.tumboon_list {
        if tumboon.songboon_id == songboon_id {
          tumboon_list.push(tumboon.clone());
        }
      }
      tumboon_list
    }

    #[ink(message)]
    pub fn tumboon_list_count(&self) -> u32 {
      self.tumboon_list.len() as u32
    }

    #[ink(message)]
    pub fn tumboon_list_count_by_songboon_id(&self, songboon_id: u32) -> u32 {
      let mut count = 0;
      for tumboon in &self.tumboon_list {
        if tumboon.songboon_id == songboon_id {
          count += 1;
        }
      }
      count as u32
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    // Alias `ink_lang` so we can use `ink::test`.
    use ink_lang as ink;

    #[ink::test]
    fn default_works() {
      let contract = Dsongboon::default();
      assert_eq!(contract.songboon_list_count(), 0);
      assert_eq!(contract.tumboon_list_count(), 0);
    }

    #[ink::test]
    fn songboon_works() {
      let mut contract = Dsongboon::default();
      assert_eq!(contract.songboon_list_count(), 0);

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
          Some(Certificate {
            certificate_position: "aaa".to_string(),
            certificate_date: "aaa".to_string(),
            file_url: "aaa".to_string(),
            file_hash: "aaa".to_string(),
            signature_url: "aaa".to_string(),
            signature_hash: "aaa".to_string(),
          }),
          Some(Certificate {
            certificate_position: "bbb".to_string(),
            certificate_date: "bbb".to_string(),
            file_url: "bbb".to_string(),
            file_hash: "bbb".to_string(),
            signature_url: "bbb".to_string(),
            signature_hash: "bbb".to_string(),
          }),
          None,
        ],
      });
      assert_eq!(contract.songboon_list_count(), 1);

      contract.add_songboon(Songboon {
        id: 2,
        donate_req_number: "abc-123".to_string(),
        donate_req_topic: "ขอทุนการศึกษา".to_string(),
        donate_req_detail: "ขอทุนการศึกษา ขอทุนการศึกษา ขอทุนการศึกษา".to_string(),
        donate_req_hashtag: Option::from("#ทุนการศึกษา".to_string()),
        donate_req_by: 1,
        donate_req_by_name: "สมศรี แองเจิ้ลไทม์".to_string(),
        donate_req_by_id_card_number: "1234567890123".to_string(),
        address: "11/13 ซ.งามวงศ์วาน 59 ลาดยาว".to_string(),
        province: "กรุงเทพมหานคร".to_string(),
        is_organization: true,
        organization: Option::from(0),
        organization_name: Option::from("มูลนิธิกระจกเงา".to_string()),
        donate_category: "children".to_string(),
        donate_req_date: "2020-01-14 13:00:00".to_string(),
        donate_doc_expire_time: "2020-02-28 23:59:59".to_string(),
        donate_doc_status: "OPE".to_string(),
        total_req_amount: 10000,
        account_bank: Option::from("ธนาคารกรุงเทพ".to_string()),
        account_number: None,
        account_name: "สมศรี แองเจิ้ลไทม์".to_string(),
        account_promptpay_nid: None,
        account_promptpay_phone: Option::from("0850581776".to_string()),
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
      });
      /*contract.add_songboon(
        2,
        "สมศรี แองเจิ้ลไทม์".to_string(),
        "มูลนิธิพระมหาไถ่".to_string(),
        "1234567890".to_string(),
        "สมศรี แองเจิ้ลไทม์".to_string(),
        "ธนาคารกสิกรไทย".to_string(),
        20000,
        "2020-01-14 13:00:00".to_string(),
        "2020-02-28 23:59:59".to_string(),
        "children".to_string(),
        "e0d123e5f316bef78bfdf5a008837577".to_string(),
        "e0d123e5f316bef78bfdf5a008837577.pdf".to_string(),
        "35d91262b3c3ec8841b54169588c97f7".to_string(),
        "35d91262b3c3ec8841b54169588c97f7.pdf".to_string(),
      );*/
      assert_eq!(contract.songboon_list_count(), 2);
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
      assert!(contract.songboon_list[0].certificates[0].is_none());
      let certificate = Certificate {
        certificate_position: "certificate_position".to_string(),
        certificate_date: "certificate_date".to_string(),
        file_url: "certificate_doc_url".to_string(),
        file_hash: "certificate_doc_hash".to_string(),
        signature_url: "certificate_signature_url".to_string(),
        signature_hash: "certificate_signature_hash".to_string(),
      };
      contract.add_certificate(1, certificate);
      assert!(!contract.songboon_list[0].certificates[0].is_none());
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
      assert_eq!(contract.songboon_list[0].donate_doc_status, "OPE".to_string());
      contract.update_songboon_status(1, "CLO".to_string());
      assert_eq!(contract.songboon_list[0].donate_doc_status, "CLO".to_string());
    }

    #[ink::test]
    fn tumboon_works() {
      let mut contract = Dsongboon::default();
      assert_eq!(contract.tumboon_list_count(), 0);

      contract.add_tumboon(Tumboon {
        id: 1,
        songboon_id: 1,
        move_amount: 500,
        move_date: "2020-01-14 19:45:00".to_string(),
        donate_by: 1,
        donate_by_name: "เจริญพร แองเจิ้ลไทม์".to_string(),
        donate_by_info_display: 1,
        slip_file_url: "xxx".to_string(),
        slip_file_hash: "2d86c4246f3c0eb516628bf324d6b9a3".to_string(),
        slip_pay_amount: 500,
        slip_pay_ref: "1234567890".to_string(),
        slip_pay_time: "2020-01-14 19:45:00".to_string(),
        slip_pay_from_bank: "กสิกรไทย".to_string(),
        slip_pay_from_account: "0123456789".to_string(),
        slip_pay_from_name: "เจริญพร แองเจิ้ลไทม์".to_string(),
        slip_pay_to_bank: Option::from("ไทยพาณิชย์".to_string()),
        slip_pay_to_account: "9876543211".to_string(),
        slip_pay_to_name: "สมบุญ แองเจิ้ลไทม์".to_string(),
        total_req_amount_history_before: 10000,
        total_receive_amount_history_before: 0,
        total_remain_amount_history_before: 10000,
        total_req_amount_history_after: 10000,
        total_receive_amount_history_after: 500,
        total_remain_amount_history_after: 9500,
      });
      assert_eq!(contract.tumboon_list_count(), 1);
      assert_eq!(contract.tumboon_list_count_by_songboon_id(1), 1);

      contract.add_tumboon(Tumboon {
        id: 2,
        songboon_id: 1,
        move_amount: 1000,
        move_date: "2020-01-14 19:45:00".to_string(),
        donate_by: 2,
        donate_by_name: "เจริญบุญ แองเจิ้ลไทม์".to_string(),
        donate_by_info_display: 1,
        slip_file_url: "xxx".to_string(),
        slip_file_hash: "66bd00e43ff8b932c14140472c4b8cc6".to_string(),
        slip_pay_amount: 1000,
        slip_pay_ref: "1234567890".to_string(),
        slip_pay_time: "2020-01-14 19:45:00".to_string(),
        slip_pay_from_bank: "กสิกรไทย".to_string(),
        slip_pay_from_account: "0123456789".to_string(),
        slip_pay_from_name: "เจริญบุญ แองเจิ้ลไทม์".to_string(),
        slip_pay_to_bank: Option::from("ไทยพาณิชย์".to_string()),
        slip_pay_to_account: "9876543211".to_string(),
        slip_pay_to_name: "สมบุญ แองเจิ้ลไทม์".to_string(),
        total_req_amount_history_before: 10000,
        total_receive_amount_history_before: 500,
        total_remain_amount_history_before: 9500,
        total_req_amount_history_after: 10000,
        total_receive_amount_history_after: 1500,
        total_remain_amount_history_after: 8500,
      });
      assert_eq!(contract.tumboon_list_count(), 2);
      assert_eq!(contract.tumboon_list_count_by_songboon_id(1), 2);

      contract.add_tumboon(Tumboon {
        id: 3,
        songboon_id: 2,
        move_amount: 1500,
        move_date: "2020-01-14 19:45:00".to_string(),
        donate_by: 2,
        donate_by_name: "เจริญศรี แองเจิ้ลไทม์".to_string(),
        donate_by_info_display: 0,
        slip_file_url: "xxx".to_string(),
        slip_file_hash: "14140472c4b8cc666bd00e43ff8b932c".to_string(),
        slip_pay_amount: 1500,
        slip_pay_ref: "1234567890".to_string(),
        slip_pay_time: "2020-01-14 19:45:00".to_string(),
        slip_pay_from_bank: "กสิกรไทย".to_string(),
        slip_pay_from_account: "0123456789".to_string(),
        slip_pay_from_name: "เจริญศรี แองเจิ้ลไทม์".to_string(),
        slip_pay_to_bank: Option::from("ไทยพาณิชย์".to_string()),
        slip_pay_to_account: "9876543211".to_string(),
        slip_pay_to_name: "สมบุญ แองเจิ้ลไทม์".to_string(),
        total_req_amount_history_before: 5000,
        total_receive_amount_history_before: 0,
        total_remain_amount_history_before: 5000,
        total_req_amount_history_after: 5000,
        total_receive_amount_history_after: 1500,
        total_remain_amount_history_after: 3500,
      });
      assert_eq!(contract.tumboon_list_count(), 3);
      assert_eq!(contract.tumboon_list_count_by_songboon_id(1), 2);
    }

    /*#[ink::test]
    fn get_songboon_works() {
      let mut contract = DsongboonContract::default();
      assert_eq!(contract.songboon_list_count(), 0);

      contract.add_songboon(
        1,
        "สมบุญ ณ แองเจิ้ลไทม์".to_string(),
        "มูลนิธิกระจกเงา".to_string(),
        "1234567890".to_string(),
        "สมบุญ ณ แองเจิ้ลไทม์".to_string(),
        "กสิกรไทย".to_string(),
        10000,
        "2020-01-14 13:00:00".to_string(),
        "2020-02-28 23:59:59".to_string(),
        "children".to_string(),
        "e0d123e5f316bef78bfdf5a008837577".to_string(),
        "e0d123e5f316bef78bfdf5a008837577.pdf".to_string(),
        "35d91262b3c3ec8841b54169588c97f7".to_string(),
        "35d91262b3c3ec8841b54169588c97f7.pdf".to_string(),
      );

      assert_eq!(contract.songboon_list_count(), 1);
      assert_eq!(contract.get_songboon(0).id, 1);
      assert_eq!(contract.get_songboon(0).recipient_name, "สมบุญ ณ แองเจิ้ลไทม์".to_string());
      assert_eq!(contract.get_songboon(0).request_amount, 10000);

      let songboon = contract.get_songboon(0);
      assert_eq!(songboon.id, 1);
      assert_eq!(songboon.recipient_name, "สมบุญ ณ แองเจิ้ลไทม์".to_string());
      assert_eq!(songboon.request_amount, 10000);
    }*/
  }
}
