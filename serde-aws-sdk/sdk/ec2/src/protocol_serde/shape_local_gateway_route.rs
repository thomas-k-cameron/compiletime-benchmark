// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_local_gateway_route(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::LocalGatewayRoute, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LocalGatewayRoute::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("destinationCidrBlock") /* DestinationCidrBlock com.amazonaws.ec2#LocalGatewayRoute$DestinationCidrBlock */ =>  {
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
            s if s.matches("localGatewayVirtualInterfaceGroupId") /* LocalGatewayVirtualInterfaceGroupId com.amazonaws.ec2#LocalGatewayRoute$LocalGatewayVirtualInterfaceGroupId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_virtual_interface_group_id(var_2);
            }
            ,
            s if s.matches("type") /* Type com.amazonaws.ec2#LocalGatewayRoute$Type */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::LocalGatewayRouteType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LocalGatewayRouteType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_3);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#LocalGatewayRoute$State */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::LocalGatewayRouteState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LocalGatewayRouteState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_4);
            }
            ,
            s if s.matches("localGatewayRouteTableId") /* LocalGatewayRouteTableId com.amazonaws.ec2#LocalGatewayRoute$LocalGatewayRouteTableId */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_route_table_id(var_5);
            }
            ,
            s if s.matches("localGatewayRouteTableArn") /* LocalGatewayRouteTableArn com.amazonaws.ec2#LocalGatewayRoute$LocalGatewayRouteTableArn */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_route_table_arn(var_6);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#LocalGatewayRoute$OwnerId */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_7);
            }
            ,
            s if s.matches("subnetId") /* SubnetId com.amazonaws.ec2#LocalGatewayRoute$SubnetId */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_id(var_8);
            }
            ,
            s if s.matches("coipPoolId") /* CoipPoolId com.amazonaws.ec2#LocalGatewayRoute$CoipPoolId */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_coip_pool_id(var_9);
            }
            ,
            s if s.matches("networkInterfaceId") /* NetworkInterfaceId com.amazonaws.ec2#LocalGatewayRoute$NetworkInterfaceId */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_interface_id(var_10);
            }
            ,
            s if s.matches("destinationPrefixListId") /* DestinationPrefixListId com.amazonaws.ec2#LocalGatewayRoute$DestinationPrefixListId */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_destination_prefix_list_id(var_11);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
