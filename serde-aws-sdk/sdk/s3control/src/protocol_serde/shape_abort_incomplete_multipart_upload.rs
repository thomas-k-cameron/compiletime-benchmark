// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_abort_incomplete_multipart_upload(
    input: &crate::types::AbortIncompleteMultipartUpload,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if input.days_after_initiation != 0 {
        let mut inner_writer = scope.start_el("DaysAfterInitiation").finish();
        inner_writer
            .data(aws_smithy_types::primitive::Encoder::from(input.days_after_initiation).encode());
    }
    scope.finish();
    Ok(())
}

pub fn de_abort_incomplete_multipart_upload(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::AbortIncompleteMultipartUpload, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AbortIncompleteMultipartUpload::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DaysAfterInitiation") /* DaysAfterInitiation com.amazonaws.s3control#AbortIncompleteMultipartUpload$DaysAfterInitiation */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3control#DaysAfterInitiation`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_days_after_initiation(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
