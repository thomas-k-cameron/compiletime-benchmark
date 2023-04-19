// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_capacity_reservation_target_response(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::CapacityReservationTargetResponse, aws_smithy_xml::decode::XmlDecodeError>
{
    #[allow(unused_mut)]
    let mut builder = crate::types::CapacityReservationTargetResponse::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("capacityReservationId") /* CapacityReservationId com.amazonaws.ec2#CapacityReservationTargetResponse$CapacityReservationId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_capacity_reservation_id(var_1);
            }
            ,
            s if s.matches("capacityReservationResourceGroupArn") /* CapacityReservationResourceGroupArn com.amazonaws.ec2#CapacityReservationTargetResponse$CapacityReservationResourceGroupArn */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_capacity_reservation_resource_group_arn(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
