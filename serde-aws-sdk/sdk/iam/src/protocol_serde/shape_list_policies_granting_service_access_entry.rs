// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_list_policies_granting_service_access_entry(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    crate::types::ListPoliciesGrantingServiceAccessEntry,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ListPoliciesGrantingServiceAccessEntry::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ServiceNamespace") /* ServiceNamespace com.amazonaws.iam#ListPoliciesGrantingServiceAccessEntry$ServiceNamespace */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_service_namespace(var_1);
            }
            ,
            s if s.matches("Policies") /* Policies com.amazonaws.iam#ListPoliciesGrantingServiceAccessEntry$Policies */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_policy_granting_service_access_list_type::de_policy_granting_service_access_list_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_policies(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}