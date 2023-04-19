// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_network_insights_access_scope_content(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::NetworkInsightsAccessScopeContent, aws_smithy_xml::decode::XmlDecodeError>
{
    #[allow(unused_mut)]
    let mut builder = crate::types::NetworkInsightsAccessScopeContent::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("networkInsightsAccessScopeId") /* NetworkInsightsAccessScopeId com.amazonaws.ec2#NetworkInsightsAccessScopeContent$NetworkInsightsAccessScopeId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_insights_access_scope_id(var_1);
            }
            ,
            s if s.matches("matchPathSet") /* MatchPaths com.amazonaws.ec2#NetworkInsightsAccessScopeContent$MatchPaths */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_access_scope_path_list::de_access_scope_path_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_match_paths(var_2);
            }
            ,
            s if s.matches("excludePathSet") /* ExcludePaths com.amazonaws.ec2#NetworkInsightsAccessScopeContent$ExcludePaths */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_access_scope_path_list::de_access_scope_path_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_exclude_paths(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}