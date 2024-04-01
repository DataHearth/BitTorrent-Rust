use rand::{distributions::Alphanumeric, Rng};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Generate a peer ID matching the specification `BEP 0020`
pub fn gen_peer_id(prefix: Option<String>) -> String {
    let prefix = if let Some(v) = prefix {
        v
    } else {
        let mut iter = VERSION.splitn(3, ".");
        let major = iter
            .next()
            .expect("version should contains 3 elements, not 0");
        let minor = iter
            .next()
            .expect("version should contains 3 elements, not 1");
        let patch = iter
            .next()
            .expect("version should contains 3 elements, not 2");

        format!("-B{}{}{}-", major, minor, patch)
    };

    let random_alphanum: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20-prefix.len())
        .map(char::from)
        .collect();

    format!("{prefix}{random_alphanum}")
}
