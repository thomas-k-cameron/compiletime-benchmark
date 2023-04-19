// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_credentials(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::Credentials, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Credentials::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AccessKeyId") /* AccessKeyId com.amazonaws.sts#Credentials$AccessKeyId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_access_key_id(var_1);
            }
            ,
            s if s.matches("SecretAccessKey") /* SecretAccessKey com.amazonaws.sts#Credentials$SecretAccessKey */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_secret_access_key(var_2);
            }
            ,
            s if s.matches("SessionToken") /* SessionToken com.amazonaws.sts#Credentials$SessionToken */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_session_token(var_3);
            }
            ,
            s if s.matches("Expiration") /* Expiration com.amazonaws.sts#Credentials$Expiration */ =>  {
                let var_4 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.sts#dateType`)"))
                        ?
                    )
                ;
                builder = builder.set_expiration(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
