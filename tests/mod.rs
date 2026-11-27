#[cfg(tests)]


mod test {
  
use uuid::Uuid;
  use super::ShortUID;
  #[test]
 pub fn uuid_v4_speed_test() {
for _ in 0..1_000_000 {
  let _uuidv4 = Uuid::new_v4();
}
   #[test]
   pub fn shortuid() {
     let _suid = ShortUID::new();
   }
 }
  todo!("Add time monitoring.");

  
}
