#![crate_type= "lib"]
#![crate_name= "tcl"]

extern crate rust_tcl_sys as ll;
#[macro_use]
extern crate bitflags;

#[doc(no_inline)]
pub use tcl::*;
#[doc(no_inline)]
pub use object::*;
#[doc(no_inline)]
pub use interpreter::*;

pub mod tcl;
pub mod object;
pub mod interpreter;
