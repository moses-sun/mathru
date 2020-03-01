#![doc(html_favicon_url = "\">
<script defer src=\"https://cdn.jsdelivr.net/npm/katex@0.10.1/dist/katex.min.js\" integrity=\"sha384-2BKqo+exmr9su6dir+qCw08N2ZKRucY4PrGQPPWU1A7FtlCGjmEGFqXCv5nyM5Ij\" crossorigin=\"anonymous\"></script>
<script>
document.addEventListener(\"DOMContentLoaded\", function () {
	let to_do = [];
	for (let e of document.getElementsByTagName(\"code\")) {
		if (e.classList.contains(\"language-math\")) {
			to_do.push(function () {
				let x = document.createElement('p');
				katex.render(e.innerText, x, {displayMode: true, throwOnError: false});
				e.parentNode.parentNode.replaceChild(x, e.parentNode);
			});
		} else {
			let n = e.nextSibling; let p = e.previousSibling;
			if (n && p && /^\\$/.test(n.data) && /\\$$/.test(p.data)) {
				to_do.push(function () {
					let n = e.nextSibling; let p = e.previousSibling;
					let x = document.createElement('span');
					katex.render(e.innerText, x, {throwOnError: false});
					e.parentNode.replaceChild(x, e);
					n.splitText(1); n.remove();
					p.splitText(p.data.length - 1).remove();
				});
			}
		}
	}
	for (let f of to_do) f();
});
</script>
<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/katex@0.10.1/dist/katex.min.css\" integrity=\"sha384-dbVIfZGuN1Yq7/1Ocstc1lUEm+AT+/rCkibIcC/OmWo5f0EA48Vf8CytHzGrSwbQ\" crossorigin=\"anonymous")]


//! # mathru
//!
//! A crate that provides  mathematics functions implemented entirely in Rust.
//!
//!
//! ## Usage
//!
//! The library usage is described well in the API documentation - including example code.
//!
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mathru = "0.6.*"
//! ```
//!
//!
//! Then import the modules and it is ready to be used:
//!
//!``` rust
//! # #[macro_use]
//! # extern crate mathru;
//! # fn main()
//! # {
//! use mathru::algebra::linear::{Vector, Matrix};
//! use mathru::algebra::linear::matrix::{Substitute};
//! // Compute the LU decomposition of a 2x2 matrix
//! let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 2.0, -3.0, -7.0]);
//! let b: Vector<f64> = vector![1.0; 3.0];
//!
//! let (l, u, p): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu().lup();
//!
//! let b_hat = &p * &b;
//!
//! let y = u.substitute_backward(b_hat);
//!
//! let x = p * l.substitute_forward(y);
//!
//! println!("{}", x);
//! # }
//!```

#[cfg(feature = "blaslapack")]
extern crate blas;
#[cfg(feature = "blaslapack")]
extern crate blas_src;
#[cfg(feature = "blaslapack")]
extern crate lapack;

//extern crate serde;

#[macro_use]
pub mod algebra;
pub mod elementary;
pub mod num;
pub mod special;
pub mod statistics;
pub mod analysis;
pub mod optimization;



