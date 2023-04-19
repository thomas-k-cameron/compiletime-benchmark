// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_failed_capacity_reservation_fleet_cancellation_result_set(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    std::vec::Vec<crate::types::FailedCapacityReservationFleetCancellationResult>,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#FailedCapacityReservationFleetCancellationResultSet$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_failed_capacity_reservation_fleet_cancellation_result::de_failed_capacity_reservation_fleet_cancellation_result(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
