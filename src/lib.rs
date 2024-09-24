#![cfg_attr(nightly, feature(doc_auto_cfg))]

pub mod ast {
    pub type AnyRes<T = ()> = Result<T, anyhow::Error>;
}

#[cfg(feature = "kernel")]
pub mod kernel;
