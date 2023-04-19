// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_noncurrent_version_transition(
    input: &crate::types::NoncurrentVersionTransition,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if input.noncurrent_days != 0 {
        let mut inner_writer = scope.start_el("NoncurrentDays").finish();
        inner_writer
            .data(aws_smithy_types::primitive::Encoder::from(input.noncurrent_days).encode());
    }
    if let Some(var_1) = &input.storage_class {
        let mut inner_writer = scope.start_el("StorageClass").finish();
        inner_writer.data(var_1.as_str());
    }
    scope.finish();
    Ok(())
}

pub fn de_noncurrent_version_transition(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::NoncurrentVersionTransition, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::NoncurrentVersionTransition::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("NoncurrentDays") /* NoncurrentDays com.amazonaws.s3control#NoncurrentVersionTransition$NoncurrentDays */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3control#Days`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_noncurrent_days(var_2);
            }
            ,
            s if s.matches("StorageClass") /* StorageClass com.amazonaws.s3control#NoncurrentVersionTransition$StorageClass */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::TransitionStorageClass, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::TransitionStorageClass::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_storage_class(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
