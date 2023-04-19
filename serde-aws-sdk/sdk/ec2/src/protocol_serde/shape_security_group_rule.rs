// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_security_group_rule(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::SecurityGroupRule, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::SecurityGroupRule::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("securityGroupRuleId") /* SecurityGroupRuleId com.amazonaws.ec2#SecurityGroupRule$SecurityGroupRuleId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_security_group_rule_id(var_1);
            }
            ,
            s if s.matches("groupId") /* GroupId com.amazonaws.ec2#SecurityGroupRule$GroupId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_group_id(var_2);
            }
            ,
            s if s.matches("groupOwnerId") /* GroupOwnerId com.amazonaws.ec2#SecurityGroupRule$GroupOwnerId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_group_owner_id(var_3);
            }
            ,
            s if s.matches("isEgress") /* IsEgress com.amazonaws.ec2#SecurityGroupRule$IsEgress */ =>  {
                let var_4 =
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
                builder = builder.set_is_egress(var_4);
            }
            ,
            s if s.matches("ipProtocol") /* IpProtocol com.amazonaws.ec2#SecurityGroupRule$IpProtocol */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ip_protocol(var_5);
            }
            ,
            s if s.matches("fromPort") /* FromPort com.amazonaws.ec2#SecurityGroupRule$FromPort */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_from_port(var_6);
            }
            ,
            s if s.matches("toPort") /* ToPort com.amazonaws.ec2#SecurityGroupRule$ToPort */ =>  {
                let var_7 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_to_port(var_7);
            }
            ,
            s if s.matches("cidrIpv4") /* CidrIpv4 com.amazonaws.ec2#SecurityGroupRule$CidrIpv4 */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cidr_ipv4(var_8);
            }
            ,
            s if s.matches("cidrIpv6") /* CidrIpv6 com.amazonaws.ec2#SecurityGroupRule$CidrIpv6 */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cidr_ipv6(var_9);
            }
            ,
            s if s.matches("prefixListId") /* PrefixListId com.amazonaws.ec2#SecurityGroupRule$PrefixListId */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix_list_id(var_10);
            }
            ,
            s if s.matches("referencedGroupInfo") /* ReferencedGroupInfo com.amazonaws.ec2#SecurityGroupRule$ReferencedGroupInfo */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_referenced_security_group::de_referenced_security_group(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_referenced_group_info(var_11);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#SecurityGroupRule$Description */ =>  {
                let var_12 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_12);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#SecurityGroupRule$Tags */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_13);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
