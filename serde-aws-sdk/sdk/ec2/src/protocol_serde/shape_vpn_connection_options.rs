// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_vpn_connection_options(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::VpnConnectionOptions, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VpnConnectionOptions::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("enableAcceleration") /* EnableAcceleration com.amazonaws.ec2#VpnConnectionOptions$EnableAcceleration */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_enable_acceleration(var_1);
            }
            ,
            s if s.matches("staticRoutesOnly") /* StaticRoutesOnly com.amazonaws.ec2#VpnConnectionOptions$StaticRoutesOnly */ =>  {
                let var_2 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_static_routes_only(var_2);
            }
            ,
            s if s.matches("localIpv4NetworkCidr") /* LocalIpv4NetworkCidr com.amazonaws.ec2#VpnConnectionOptions$LocalIpv4NetworkCidr */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_ipv4_network_cidr(var_3);
            }
            ,
            s if s.matches("remoteIpv4NetworkCidr") /* RemoteIpv4NetworkCidr com.amazonaws.ec2#VpnConnectionOptions$RemoteIpv4NetworkCidr */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_remote_ipv4_network_cidr(var_4);
            }
            ,
            s if s.matches("localIpv6NetworkCidr") /* LocalIpv6NetworkCidr com.amazonaws.ec2#VpnConnectionOptions$LocalIpv6NetworkCidr */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_ipv6_network_cidr(var_5);
            }
            ,
            s if s.matches("remoteIpv6NetworkCidr") /* RemoteIpv6NetworkCidr com.amazonaws.ec2#VpnConnectionOptions$RemoteIpv6NetworkCidr */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_remote_ipv6_network_cidr(var_6);
            }
            ,
            s if s.matches("outsideIpAddressType") /* OutsideIpAddressType com.amazonaws.ec2#VpnConnectionOptions$OutsideIpAddressType */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_outside_ip_address_type(var_7);
            }
            ,
            s if s.matches("transportTransitGatewayAttachmentId") /* TransportTransitGatewayAttachmentId com.amazonaws.ec2#VpnConnectionOptions$TransportTransitGatewayAttachmentId */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transport_transit_gateway_attachment_id(var_8);
            }
            ,
            s if s.matches("tunnelInsideIpVersion") /* TunnelInsideIpVersion com.amazonaws.ec2#VpnConnectionOptions$TunnelInsideIpVersion */ =>  {
                let var_9 =
                    Some(
                        Result::<crate::types::TunnelInsideIpVersion, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::TunnelInsideIpVersion::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_tunnel_inside_ip_version(var_9);
            }
            ,
            s if s.matches("tunnelOptionSet") /* TunnelOptions com.amazonaws.ec2#VpnConnectionOptions$TunnelOptions */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_tunnel_options_list::de_tunnel_options_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tunnel_options(var_10);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
