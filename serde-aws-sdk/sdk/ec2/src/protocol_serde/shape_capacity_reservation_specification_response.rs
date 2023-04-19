// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_capacity_reservation_specification_response(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    crate::types::CapacityReservationSpecificationResponse,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    #[allow(unused_mut)]
    let mut builder = crate::types::CapacityReservationSpecificationResponse::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("capacityReservationPreference") /* CapacityReservationPreference com.amazonaws.ec2#CapacityReservationSpecificationResponse$CapacityReservationPreference */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::CapacityReservationPreference, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::CapacityReservationPreference::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_capacity_reservation_preference(var_1);
            }
            ,
            s if s.matches("capacityReservationTarget") /* CapacityReservationTarget com.amazonaws.ec2#CapacityReservationSpecificationResponse$CapacityReservationTarget */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_capacity_reservation_target_response::de_capacity_reservation_target_response(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_capacity_reservation_target(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
