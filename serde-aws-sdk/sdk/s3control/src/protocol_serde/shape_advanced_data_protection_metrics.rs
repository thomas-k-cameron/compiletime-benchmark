// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_advanced_data_protection_metrics(
    input: &crate::types::AdvancedDataProtectionMetrics,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if input.is_enabled {
        let mut inner_writer = scope.start_el("IsEnabled").finish();
        inner_writer.data(aws_smithy_types::primitive::Encoder::from(input.is_enabled).encode());
    }
    scope.finish();
    Ok(())
}

pub fn de_advanced_data_protection_metrics(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::AdvancedDataProtectionMetrics, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AdvancedDataProtectionMetrics::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("IsEnabled") /* IsEnabled com.amazonaws.s3control#AdvancedDataProtectionMetrics$IsEnabled */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3control#IsEnabled`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_enabled(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
