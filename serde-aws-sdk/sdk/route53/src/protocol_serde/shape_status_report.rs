// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_status_report(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::StatusReport, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::StatusReport::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Status") /* Status com.amazonaws.route53#StatusReport$Status */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_1);
            }
            ,
            s if s.matches("CheckedTime") /* CheckedTime com.amazonaws.route53#StatusReport$CheckedTime */ =>  {
                let var_2 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.route53#TimeStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_checked_time(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}