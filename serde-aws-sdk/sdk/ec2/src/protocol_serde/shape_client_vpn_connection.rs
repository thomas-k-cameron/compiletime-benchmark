// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_client_vpn_connection(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ClientVpnConnection, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ClientVpnConnection::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("clientVpnEndpointId") /* ClientVpnEndpointId com.amazonaws.ec2#ClientVpnConnection$ClientVpnEndpointId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_client_vpn_endpoint_id(var_1);
            }
            ,
            s if s.matches("timestamp") /* Timestamp com.amazonaws.ec2#ClientVpnConnection$Timestamp */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_timestamp(var_2);
            }
            ,
            s if s.matches("connectionId") /* ConnectionId com.amazonaws.ec2#ClientVpnConnection$ConnectionId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_connection_id(var_3);
            }
            ,
            s if s.matches("username") /* Username com.amazonaws.ec2#ClientVpnConnection$Username */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_username(var_4);
            }
            ,
            s if s.matches("connectionEstablishedTime") /* ConnectionEstablishedTime com.amazonaws.ec2#ClientVpnConnection$ConnectionEstablishedTime */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_connection_established_time(var_5);
            }
            ,
            s if s.matches("ingressBytes") /* IngressBytes com.amazonaws.ec2#ClientVpnConnection$IngressBytes */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ingress_bytes(var_6);
            }
            ,
            s if s.matches("egressBytes") /* EgressBytes com.amazonaws.ec2#ClientVpnConnection$EgressBytes */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_egress_bytes(var_7);
            }
            ,
            s if s.matches("ingressPackets") /* IngressPackets com.amazonaws.ec2#ClientVpnConnection$IngressPackets */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ingress_packets(var_8);
            }
            ,
            s if s.matches("egressPackets") /* EgressPackets com.amazonaws.ec2#ClientVpnConnection$EgressPackets */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_egress_packets(var_9);
            }
            ,
            s if s.matches("clientIp") /* ClientIp com.amazonaws.ec2#ClientVpnConnection$ClientIp */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_client_ip(var_10);
            }
            ,
            s if s.matches("commonName") /* CommonName com.amazonaws.ec2#ClientVpnConnection$CommonName */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_common_name(var_11);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2#ClientVpnConnection$Status */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_client_vpn_connection_status::de_client_vpn_connection_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status(var_12);
            }
            ,
            s if s.matches("connectionEndTime") /* ConnectionEndTime com.amazonaws.ec2#ClientVpnConnection$ConnectionEndTime */ =>  {
                let var_13 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_connection_end_time(var_13);
            }
            ,
            s if s.matches("postureComplianceStatusSet") /* PostureComplianceStatuses com.amazonaws.ec2#ClientVpnConnection$PostureComplianceStatuses */ =>  {
                let var_14 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_posture_compliance_statuses(var_14);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}