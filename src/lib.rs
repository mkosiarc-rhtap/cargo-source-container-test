//! Minimal crate that actually references its dependency so the manifest is a
//! realistic cargo project. The container build does NOT need to compile this;
//! Hermeto's prefetch only needs a valid Cargo.toml + Cargo.lock to vendor deps.

/// Render an integer using the itoa crate.
pub fn render(n: u64) -> String {
    let mut buf = itoa::Buffer::new();
    buf.format(n).to_string()
}
