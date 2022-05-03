/// ## BIT!
/// Make a shifted bitfield with ease using bitflags! and BIT!<br>
/// Example:
/// ```
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
/// ```
#[macro_export]
macro_rules! BIT {
    ($arg:expr) => {
        (1 << $arg)
    };
}
/// ## CAST!
/// Dyn Traits which have as_any() implemented can be casted with this macro
/// Example:
/// ```
/// trait Foo {
///  fn as_any(&self) -> &dyn std::any::Any; 
/// }
/// struct Bar{} 
/// impl Foo for Bar {
///   fn as_any(&self) -> &dyn std::any::Any { return self; }
/// }
/// fn main() {
///   let f: &dyn Foo = &Bar{};
///   CAST!(f, Bar);
/// }
/// ```
#[macro_export]
macro_rules! CAST {
    ($arg:expr, $label:tt) => {
        $arg.as_any().downcast_ref::<$label>().expect("CAST FAILED!");
    };
}
#[macro_export]
macro_rules! CASTMUT {
    ($arg:expr, $label:tt) => {
        $arg.as_any_mut().downcast_mut::<$label>().expect("CAST FAILED!");
    };
}
#[macro_export]
macro_rules! CASTIMPL {
    () => {
        fn as_any(&self) -> &dyn std::any::Any { return self; }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any { return self; }
    };
}
#[macro_export]
macro_rules! CASTIMPLTRAIT {
    () => {
        fn as_any(&self) -> &dyn std::any::Any;
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    };
}
#[macro_export]
macro_rules! TR_ASSERT {
    ($arg:expr, $($message:tt)*) => {
        if $arg == 0 {
            TR_ERROR!("Assertion Failed: {}", $($message)*);
            panic!();
        }
    };
}
/// # Smart Pointers
/// Currently only implemented using std, but considering to write into own memory manager
/// ## UPtr
/// Unique Pointer<br>
/// Currently only an alias for and Box<T>, -''-
#[allow(unused)]
pub type UPtr<T> = std::boxed::Box<T>;
/// ## SPtr
/// Shared Pointer<br>
/// Currently only an alias for and Rc<T>, -''-
#[allow(unused)]
pub type SPtr<T> = std::rc::Rc<T>;
/// ## Vector
/// Vector<br>
/// Currently only an alias for Vec<T>, -''-
pub type Vector<T> = Vec<T>;
