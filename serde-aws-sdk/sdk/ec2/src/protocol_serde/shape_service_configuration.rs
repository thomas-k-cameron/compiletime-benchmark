// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_service_configuration(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ServiceConfiguration, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ServiceConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("serviceType") /* ServiceType com.amazonaws.ec2#ServiceConfiguration$ServiceType */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_service_type_detail_set::de_service_type_detail_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_service_type(var_1);
            }
            ,
            s if s.matches("serviceId") /* ServiceId com.amazonaws.ec2#ServiceConfiguration$ServiceId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_service_id(var_2);
            }
            ,
            s if s.matches("serviceName") /* ServiceName com.amazonaws.ec2#ServiceConfiguration$ServiceName */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_service_name(var_3);
            }
            ,
            s if s.matches("serviceState") /* ServiceState com.amazonaws.ec2#ServiceConfiguration$ServiceState */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::ServiceState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ServiceState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_service_state(var_4);
            }
            ,
            s if s.matches("availabilityZoneSet") /* AvailabilityZones com.amazonaws.ec2#ServiceConfiguration$AvailabilityZones */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_zones(var_5);
            }
            ,
            s if s.matches("acceptanceRequired") /* AcceptanceRequired com.amazonaws.ec2#ServiceConfiguration$AcceptanceRequired */ =>  {
                let var_6 =
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
                builder = builder.set_acceptance_required(var_6);
            }
            ,
            s if s.matches("managesVpcEndpoints") /* ManagesVpcEndpoints com.amazonaws.ec2#ServiceConfiguration$ManagesVpcEndpoints */ =>  {
                let var_7 =
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
                builder = builder.set_manages_vpc_endpoints(var_7);
            }
            ,
            s if s.matches("networkLoadBalancerArnSet") /* NetworkLoadBalancerArns com.amazonaws.ec2#ServiceConfiguration$NetworkLoadBalancerArns */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_network_load_balancer_arns(var_8);
            }
            ,
            s if s.matches("gatewayLoadBalancerArnSet") /* GatewayLoadBalancerArns com.amazonaws.ec2#ServiceConfiguration$GatewayLoadBalancerArns */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_gateway_load_balancer_arns(var_9);
            }
            ,
            s if s.matches("supportedIpAddressTypeSet") /* SupportedIpAddressTypes com.amazonaws.ec2#ServiceConfiguration$SupportedIpAddressTypes */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_supported_ip_address_types::de_supported_ip_address_types(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_supported_ip_address_types(var_10);
            }
            ,
            s if s.matches("baseEndpointDnsNameSet") /* BaseEndpointDnsNames com.amazonaws.ec2#ServiceConfiguration$BaseEndpointDnsNames */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_base_endpoint_dns_names(var_11);
            }
            ,
            s if s.matches("privateDnsName") /* PrivateDnsName com.amazonaws.ec2#ServiceConfiguration$PrivateDnsName */ =>  {
                let var_12 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_private_dns_name(var_12);
            }
            ,
            s if s.matches("privateDnsNameConfiguration") /* PrivateDnsNameConfiguration com.amazonaws.ec2#ServiceConfiguration$PrivateDnsNameConfiguration */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_private_dns_name_configuration::de_private_dns_name_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_private_dns_name_configuration(var_13);
            }
            ,
            s if s.matches("payerResponsibility") /* PayerResponsibility com.amazonaws.ec2#ServiceConfiguration$PayerResponsibility */ =>  {
                let var_14 =
                    Some(
                        Result::<crate::types::PayerResponsibility, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::PayerResponsibility::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_payer_responsibility(var_14);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#ServiceConfiguration$Tags */ =>  {
                let var_15 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_15);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
