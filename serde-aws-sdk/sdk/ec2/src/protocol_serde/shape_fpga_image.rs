// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_fpga_image(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::FpgaImage, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::FpgaImage::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("fpgaImageId") /* FpgaImageId com.amazonaws.ec2#FpgaImage$FpgaImageId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_fpga_image_id(var_1);
            }
            ,
            s if s.matches("fpgaImageGlobalId") /* FpgaImageGlobalId com.amazonaws.ec2#FpgaImage$FpgaImageGlobalId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_fpga_image_global_id(var_2);
            }
            ,
            s if s.matches("name") /* Name com.amazonaws.ec2#FpgaImage$Name */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_3);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#FpgaImage$Description */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_4);
            }
            ,
            s if s.matches("shellVersion") /* ShellVersion com.amazonaws.ec2#FpgaImage$ShellVersion */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_shell_version(var_5);
            }
            ,
            s if s.matches("pciId") /* PciId com.amazonaws.ec2#FpgaImage$PciId */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_pci_id::de_pci_id(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_pci_id(var_6);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#FpgaImage$State */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_fpga_image_state::de_fpga_image_state(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_state(var_7);
            }
            ,
            s if s.matches("createTime") /* CreateTime com.amazonaws.ec2#FpgaImage$CreateTime */ =>  {
                let var_8 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_create_time(var_8);
            }
            ,
            s if s.matches("updateTime") /* UpdateTime com.amazonaws.ec2#FpgaImage$UpdateTime */ =>  {
                let var_9 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_update_time(var_9);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#FpgaImage$OwnerId */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_10);
            }
            ,
            s if s.matches("ownerAlias") /* OwnerAlias com.amazonaws.ec2#FpgaImage$OwnerAlias */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_alias(var_11);
            }
            ,
            s if s.matches("productCodes") /* ProductCodes com.amazonaws.ec2#FpgaImage$ProductCodes */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_product_code_list::de_product_code_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_product_codes(var_12);
            }
            ,
            s if s.matches("tags") /* Tags com.amazonaws.ec2#FpgaImage$Tags */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_13);
            }
            ,
            s if s.matches("public") /* Public com.amazonaws.ec2#FpgaImage$Public */ =>  {
                let var_14 =
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
                builder = builder.set_public(var_14);
            }
            ,
            s if s.matches("dataRetentionSupport") /* DataRetentionSupport com.amazonaws.ec2#FpgaImage$DataRetentionSupport */ =>  {
                let var_15 =
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
                builder = builder.set_data_retention_support(var_15);
            }
            ,
            s if s.matches("instanceTypes") /* InstanceTypes com.amazonaws.ec2#FpgaImage$InstanceTypes */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_instance_types_list::de_instance_types_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_types(var_16);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
