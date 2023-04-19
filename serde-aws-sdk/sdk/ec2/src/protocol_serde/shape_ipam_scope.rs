// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_ipam_scope(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::IpamScope, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::IpamScope::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#IpamScope$OwnerId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_1);
            }
            ,
            s if s.matches("ipamScopeId") /* IpamScopeId com.amazonaws.ec2#IpamScope$IpamScopeId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_scope_id(var_2);
            }
            ,
            s if s.matches("ipamScopeArn") /* IpamScopeArn com.amazonaws.ec2#IpamScope$IpamScopeArn */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_scope_arn(var_3);
            }
            ,
            s if s.matches("ipamArn") /* IpamArn com.amazonaws.ec2#IpamScope$IpamArn */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_arn(var_4);
            }
            ,
            s if s.matches("ipamRegion") /* IpamRegion com.amazonaws.ec2#IpamScope$IpamRegion */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_region(var_5);
            }
            ,
            s if s.matches("ipamScopeType") /* IpamScopeType com.amazonaws.ec2#IpamScope$IpamScopeType */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::types::IpamScopeType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::IpamScopeType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_scope_type(var_6);
            }
            ,
            s if s.matches("isDefault") /* IsDefault com.amazonaws.ec2#IpamScope$IsDefault */ =>  {
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
                builder = builder.set_is_default(var_7);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#IpamScope$Description */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_8);
            }
            ,
            s if s.matches("poolCount") /* PoolCount com.amazonaws.ec2#IpamScope$PoolCount */ =>  {
                let var_9 =
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
                builder = builder.set_pool_count(var_9);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#IpamScope$State */ =>  {
                let var_10 =
                    Some(
                        Result::<crate::types::IpamScopeState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::IpamScopeState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_10);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#IpamScope$Tags */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_11);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
