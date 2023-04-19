// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_transit_gateway_attachment_propagation(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::TransitGatewayAttachmentPropagation, aws_smithy_xml::decode::XmlDecodeError>
{
    #[allow(unused_mut)]
    let mut builder = crate::types::TransitGatewayAttachmentPropagation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("transitGatewayRouteTableId") /* TransitGatewayRouteTableId com.amazonaws.ec2#TransitGatewayAttachmentPropagation$TransitGatewayRouteTableId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_route_table_id(var_1);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#TransitGatewayAttachmentPropagation$State */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::TransitGatewayPropagationState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::TransitGatewayPropagationState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}