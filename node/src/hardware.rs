use std::sync::OnceLock;

use sc_sysinfo::Requirements;

/// The hardware requirements as measured on reference hardware.
///
/// These values are provided by Horizenlabs, however it is possible
/// to use your own requirements if you are running a custom chain.
pub fn zkv_reference_hardware() -> &'static Requirements {
    static REFERENCE_HW: OnceLock<Requirements> = OnceLock::new();
    REFERENCE_HW.get_or_init(|| {
        let raw = include_bytes!("reference_hardware.json").as_slice();
        serde_json::from_slice(raw).expect("Hardcoded data is known good; qed")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sc_sysinfo::Requirements;

    /// `zkv_reference_hardware()` can be decoded.
    #[test]
    fn json_static_data() {
        let raw = serde_json::to_string(zkv_reference_hardware()).unwrap();
        let decoded: Requirements = serde_json::from_str(&raw).unwrap();

        assert_eq!(&decoded, zkv_reference_hardware());
    }
}
