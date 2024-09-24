#![cfg_attr(nightly, feature(doc_auto_cfg))]

pub mod ast {
    pub type AnyRes<T = (), E = anyhow::Error> = Result<T, E>;
}

#[cfg(feature = "kernel")]
pub mod kernel;
