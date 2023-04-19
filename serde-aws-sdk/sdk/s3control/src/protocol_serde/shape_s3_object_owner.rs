// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_object_owner(
    input: &crate::types::S3ObjectOwner,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.id {
        let mut inner_writer = scope.start_el("ID").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.display_name {
        let mut inner_writer = scope.start_el("DisplayName").finish();
        inner_writer.data(var_2.as_str());
    }
    scope.finish();
    Ok(())
}

pub fn de_s3_object_owner(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::S3ObjectOwner, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::S3ObjectOwner::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ID") /* ID com.amazonaws.s3control#S3ObjectOwner$ID */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_3);
            }
            ,
            s if s.matches("DisplayName") /* DisplayName com.amazonaws.s3control#S3ObjectOwner$DisplayName */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_display_name(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
