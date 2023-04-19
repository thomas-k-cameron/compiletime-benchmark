// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_vpn_static_route(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::VpnStaticRoute, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VpnStaticRoute::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("destinationCidrBlock") /* DestinationCidrBlock com.amazonaws.ec2#VpnStaticRoute$DestinationCidrBlock */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_destination_cidr_block(var_1);
            }
            ,
            s if s.matches("source") /* Source com.amazonaws.ec2#VpnStaticRoute$Source */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::VpnStaticRouteSource, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::VpnStaticRouteSource::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_source(var_2);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#VpnStaticRoute$State */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::VpnState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::VpnState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
