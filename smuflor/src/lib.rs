//! Load a [SMuFL](https://www.smufl.org/) compliant music fonts as vector
//! graphics in pure Rust with advanced score layout.  When you want to render a
//! score, Smuflor gives you an iterator over
//! [footile](https://crates.io/crates/footile) `PathOps`, which you can easily
//! pass right into footile.

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://studio.scorefall.com/favicon.png",
    html_favicon_url = "https://studio.scorefall.com/favicon.png"
)]

mod view;

pub use view::View;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
