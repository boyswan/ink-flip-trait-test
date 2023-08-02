#[derive(Default, Debug)]
#[ink::storage_item]
pub struct FlipStorage {
  value: bool
}

pub trait FlipStorageAccess {
    fn storage(&mut self) -> &mut FlipStorage;
}


#[ink::trait_definition]
pub trait Flip {
    #[ink(message)]
    fn flip(&mut self);

    #[ink(message)]
    fn get(&mut self) -> bool;
}


pub trait FlipDefault: FlipStorageAccess {
    /// Flips the current value of the Flipper's boolean.
    fn flip(&mut self) {
       self.storage().value = !self.storage().value;
    }

    /// Returns the current value of the Flipper's boolean.
    fn get(&mut self) -> bool {
      self.storage().value
    }
}
