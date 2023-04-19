// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_access_scope_analysis_finding(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::AccessScopeAnalysisFinding, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AccessScopeAnalysisFinding::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("networkInsightsAccessScopeAnalysisId") /* NetworkInsightsAccessScopeAnalysisId com.amazonaws.ec2#AccessScopeAnalysisFinding$NetworkInsightsAccessScopeAnalysisId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_insights_access_scope_analysis_id(var_1);
            }
            ,
            s if s.matches("networkInsightsAccessScopeId") /* NetworkInsightsAccessScopeId com.amazonaws.ec2#AccessScopeAnalysisFinding$NetworkInsightsAccessScopeId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_insights_access_scope_id(var_2);
            }
            ,
            s if s.matches("findingId") /* FindingId com.amazonaws.ec2#AccessScopeAnalysisFinding$FindingId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_finding_id(var_3);
            }
            ,
            s if s.matches("findingComponentSet") /* FindingComponents com.amazonaws.ec2#AccessScopeAnalysisFinding$FindingComponents */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_path_component_list::de_path_component_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_finding_components(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}