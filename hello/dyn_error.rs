fn foo(_: &dyn std::error::Error) {}

enum E {
	A(Box<dyn std::error::Error>),
}

fn bar(e: &E) {
	match e {
		E::A(err) => {
			foo(err); // solusi: err.as_ref() atau &**err
		}
	}
}

fn main() {
}

/*
error[E0277]: the size for values of type `(dyn std::error::Error + 'static)` cannot be known at compilation time
  --> dyn_error.rs:10:8
   |
10 |             foo(err);
   |                 ^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn std::error::Error + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: required because of the requirements on the impl of `std::error::Error` for `std::boxed::Box<(dyn std::error::Error + 'static)>`
   = note: required for the cast to the object type `dyn std::error::Error`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
*/
