// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_permissions_boundary_decision_detail(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::PermissionsBoundaryDecisionDetail, aws_smithy_xml::decode::XmlDecodeError>
{
    #[allow(unused_mut)]
    let mut builder = crate::types::PermissionsBoundaryDecisionDetail::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AllowedByPermissionsBoundary") /* AllowedByPermissionsBoundary com.amazonaws.iam#PermissionsBoundaryDecisionDetail$AllowedByPermissionsBoundary */ =>  {
                let var_1 =
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
                builder = builder.set_allowed_by_permissions_boundary(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}