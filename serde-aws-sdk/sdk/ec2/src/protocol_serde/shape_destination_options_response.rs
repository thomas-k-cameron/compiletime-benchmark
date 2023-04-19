// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_destination_options_response(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::DestinationOptionsResponse, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DestinationOptionsResponse::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("fileFormat") /* FileFormat com.amazonaws.ec2#DestinationOptionsResponse$FileFormat */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::DestinationFileFormat, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::DestinationFileFormat::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_file_format(var_1);
            }
            ,
            s if s.matches("hiveCompatiblePartitions") /* HiveCompatiblePartitions com.amazonaws.ec2#DestinationOptionsResponse$HiveCompatiblePartitions */ =>  {
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
                builder = builder.set_hive_compatible_partitions(var_2);
            }
            ,
            s if s.matches("perHourPartition") /* PerHourPartition com.amazonaws.ec2#DestinationOptionsResponse$PerHourPartition */ =>  {
                let var_3 =
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
                builder = builder.set_per_hour_partition(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
