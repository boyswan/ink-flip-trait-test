#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod flip;

#[ink::contract]
mod flipper {

    use super::flip::{FlipStorage, FlipStorageAccess, FlipDefault, Flip};
    #[ink(storage)]
    pub struct Flipper {
        flip: FlipStorage,
    }


    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { flip: FlipStorage::default() }
        }
    }

    impl FlipStorageAccess for Flipper {
        fn storage(&mut self) -> &mut FlipStorage {
          &mut self.flip
        }
    }

    impl FlipDefault for Flipper {}
    impl Flip for Flipper {
        #[ink(message)]
        fn flip(&mut self) {
            FlipDefault::flip(self)
        }

        #[ink(message)]
        fn get(&mut self) -> bool {
            FlipDefault::get(self)
        }
    }
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::{Flipper, Flip};
        // use super::foo::{FlipDefault};


        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(Flip::get(&mut flipper), false);
            flipper.flip();
            assert_eq!(Flip::get(&mut flipper), true);
        }
    }

}
