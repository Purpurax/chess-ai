
// #[cfg(debug_assertions)]
// mod readable;
// #[cfg(not(debug_assertions))]
// mod optimized;
mod hardcoded;

// #[cfg(debug_assertions)]
// pub use readable::*;
// #[cfg(not(debug_assertions))]
// pub use optimized::*;
pub use hardcoded::*;
