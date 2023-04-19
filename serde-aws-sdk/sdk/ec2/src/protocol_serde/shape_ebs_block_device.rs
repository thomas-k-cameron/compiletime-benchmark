// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_ebs_block_device(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::EbsBlockDevice,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DeleteOnTermination");
    if let Some(var_2) = &input.delete_on_termination {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Iops");
    if let Some(var_4) = &input.iops {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("SnapshotId");
    if let Some(var_6) = &input.snapshot_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("VolumeSize");
    if let Some(var_8) = &input.volume_size {
        scope_7.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("VolumeType");
    if let Some(var_10) = &input.volume_type {
        scope_9.string(var_10.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("KmsKeyId");
    if let Some(var_12) = &input.kms_key_id {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("Throughput");
    if let Some(var_14) = &input.throughput {
        scope_13.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("OutpostArn");
    if let Some(var_16) = &input.outpost_arn {
        scope_15.string(var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("Encrypted");
    if let Some(var_18) = &input.encrypted {
        scope_17.boolean(*var_18);
    }
    Ok(())
}

pub fn de_ebs_block_device(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::EbsBlockDevice, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::EbsBlockDevice::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("deleteOnTermination") /* DeleteOnTermination com.amazonaws.ec2#EbsBlockDevice$DeleteOnTermination */ =>  {
                let var_19 =
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
                builder = builder.set_delete_on_termination(var_19);
            }
            ,
            s if s.matches("iops") /* Iops com.amazonaws.ec2#EbsBlockDevice$Iops */ =>  {
                let var_20 =
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
                builder = builder.set_iops(var_20);
            }
            ,
            s if s.matches("snapshotId") /* SnapshotId com.amazonaws.ec2#EbsBlockDevice$SnapshotId */ =>  {
                let var_21 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_snapshot_id(var_21);
            }
            ,
            s if s.matches("volumeSize") /* VolumeSize com.amazonaws.ec2#EbsBlockDevice$VolumeSize */ =>  {
                let var_22 =
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
                builder = builder.set_volume_size(var_22);
            }
            ,
            s if s.matches("volumeType") /* VolumeType com.amazonaws.ec2#EbsBlockDevice$VolumeType */ =>  {
                let var_23 =
                    Some(
                        Result::<crate::types::VolumeType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::VolumeType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_volume_type(var_23);
            }
            ,
            s if s.matches("kmsKeyId") /* KmsKeyId com.amazonaws.ec2#EbsBlockDevice$KmsKeyId */ =>  {
                let var_24 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_kms_key_id(var_24);
            }
            ,
            s if s.matches("throughput") /* Throughput com.amazonaws.ec2#EbsBlockDevice$Throughput */ =>  {
                let var_25 =
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
                builder = builder.set_throughput(var_25);
            }
            ,
            s if s.matches("outpostArn") /* OutpostArn com.amazonaws.ec2#EbsBlockDevice$OutpostArn */ =>  {
                let var_26 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_outpost_arn(var_26);
            }
            ,
            s if s.matches("encrypted") /* Encrypted com.amazonaws.ec2#EbsBlockDevice$Encrypted */ =>  {
                let var_27 =
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
                builder = builder.set_encrypted(var_27);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
