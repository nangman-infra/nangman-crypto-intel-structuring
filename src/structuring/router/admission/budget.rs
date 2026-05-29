use crate::hash::sha256_hex;

pub(in crate::structuring::router) fn within_escalation_budget(
    raw_event_id: &str,
    ratio: f64,
) -> bool {
    if ratio <= 0.0 {
        return false;
    }
    if ratio >= 1.0 {
        return true;
    }
    let digest = sha256_hex(raw_event_id.as_bytes());
    let Some(prefix) = digest.get(..8) else {
        return false;
    };
    let Ok(value) = u32::from_str_radix(prefix, 16) else {
        return false;
    };
    let normalized = value as f64 / u32::MAX as f64;
    normalized < ratio
}
