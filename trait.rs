
#[ink::trait_definition]
pub trait Flip {
    /// Flips the current value of the Flipper's boolean.
    #[ink(message)]
    fn flip(&mut self);

    /// Returns the current value of the Flipper's boolean.
    #[ink(message)]
    fn get(&self) -> bool;
}
