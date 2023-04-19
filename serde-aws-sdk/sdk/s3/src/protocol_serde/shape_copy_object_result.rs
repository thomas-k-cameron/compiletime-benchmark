// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_copy_object_result(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::CopyObjectResult, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::CopyObjectResult::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ETag") /* ETag com.amazonaws.s3#CopyObjectResult$ETag */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_e_tag(var_1);
            }
            ,
            s if s.matches("LastModified") /* LastModified com.amazonaws.s3#CopyObjectResult$LastModified */ =>  {
                let var_2 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.s3#LastModified`)"))
                        ?
                    )
                ;
                builder = builder.set_last_modified(var_2);
            }
            ,
            s if s.matches("ChecksumCRC32") /* ChecksumCRC32 com.amazonaws.s3#CopyObjectResult$ChecksumCRC32 */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_checksum_crc32(var_3);
            }
            ,
            s if s.matches("ChecksumCRC32C") /* ChecksumCRC32C com.amazonaws.s3#CopyObjectResult$ChecksumCRC32C */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_checksum_crc32_c(var_4);
            }
            ,
            s if s.matches("ChecksumSHA1") /* ChecksumSHA1 com.amazonaws.s3#CopyObjectResult$ChecksumSHA1 */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_checksum_sha1(var_5);
            }
            ,
            s if s.matches("ChecksumSHA256") /* ChecksumSHA256 com.amazonaws.s3#CopyObjectResult$ChecksumSHA256 */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_checksum_sha256(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
