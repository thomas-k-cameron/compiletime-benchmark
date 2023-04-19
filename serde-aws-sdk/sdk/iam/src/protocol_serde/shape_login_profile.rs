// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_login_profile(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::LoginProfile, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LoginProfile::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("UserName") /* UserName com.amazonaws.iam#LoginProfile$UserName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_user_name(var_1);
            }
            ,
            s if s.matches("CreateDate") /* CreateDate com.amazonaws.iam#LoginProfile$CreateDate */ =>  {
                let var_2 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.iam#dateType`)"))
                        ?
                    )
                ;
                builder = builder.set_create_date(var_2);
            }
            ,
            s if s.matches("PasswordResetRequired") /* PasswordResetRequired com.amazonaws.iam#LoginProfile$PasswordResetRequired */ =>  {
                let var_3 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.iam#booleanType`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_password_reset_required(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}