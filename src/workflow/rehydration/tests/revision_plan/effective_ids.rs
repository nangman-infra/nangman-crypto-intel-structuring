use super::super::super::revision::{effective_packet_family_id, effective_raw_event_id};
use super::super::fixtures::packet_with_market_status;
use crate::models::market::MarketContextStatus;

#[test]
fn effective_ids_fall_back_to_source_event_then_packet_id() {
    let mut packet = packet_with_market_status(MarketContextStatus::Pending);
    packet.packet_family_id.clear();
    packet.raw_event_id.clear();

    assert_eq!(effective_packet_family_id(&packet), "source_1");
    assert_eq!(effective_raw_event_id(&packet), "source_1");

    packet.source_event_ids.clear();

    assert_eq!(effective_packet_family_id(&packet), "packet_1");
    assert_eq!(effective_raw_event_id(&packet), "packet_1");
}

#[test]
fn effective_ids_skip_blank_source_event_ids() {
    let mut packet = packet_with_market_status(MarketContextStatus::Pending);
    packet.packet_family_id.clear();
    packet.raw_event_id.clear();
    packet.source_event_ids = vec![" ".to_owned(), "source_2".to_owned()];

    assert_eq!(effective_packet_family_id(&packet), "source_2");
    assert_eq!(effective_raw_event_id(&packet), "source_2");
}
