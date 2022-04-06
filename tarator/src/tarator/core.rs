/// ## BIT!
/// Make a shifted bitfield with ease using bitflags! and BIT!
/// Example:
/// bitflags! {
///   pub struct ExampleField: u8 {
///     const NONE        = 0;        // 0b00000000
///     const EXAMPLE1    = BIT!(0);  // 0b00000001
///     const EXAMPLE2    = BIT!(1);  // 0b00000010
///     const EXAMPLE3    = BIT!(2);  // 0b00000100
///     const EXAMPLE4    = BIT!(3);  // 0b00001000
///     const EXAMPLE6    = BIT!(5);  // 0b00100000
///     const EXAMPLE7    = BIT!(6);  // 0b01000000
///     const EXAMPLE8    = BIT!(7);  // 0b10000000
///   }
/// }
#[macro_export]
macro_rules! BIT {
    ($arg:expr) => {
        (1 << $arg)
    };
}