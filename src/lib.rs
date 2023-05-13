pub mod u8;

/// Trait for checking if a number is odd.
pub trait ThatsOdd {
    fn is_odd(&self) -> bool;
    fn not_odd(&self) -> bool {
        !self.is_odd()
    }
}
