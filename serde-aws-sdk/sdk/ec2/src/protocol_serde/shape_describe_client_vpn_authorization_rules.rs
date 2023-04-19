// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_client_vpn_authorization_rules_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesOutput, crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_client_vpn_authorization_rules_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesOutput, crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_client_vpn_authorization_rules::builders::DescribeClientVpnAuthorizationRulesOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_client_vpn_authorization_rules::de_describe_client_vpn_authorization_rules(response.body().as_ref(), output).map_err(crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_client_vpn_authorization_rules(inp: &[u8], mut builder: crate::operation::describe_client_vpn_authorization_rules::builders::DescribeClientVpnAuthorizationRulesOutputBuilder) -> Result<crate::operation::describe_client_vpn_authorization_rules::builders::DescribeClientVpnAuthorizationRulesOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeClientVpnAuthorizationRulesResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeClientVpnAuthorizationRulesResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("authorizationRule") /* AuthorizationRules com.amazonaws.ec2.synthetic#DescribeClientVpnAuthorizationRulesOutput$AuthorizationRules */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_authorization_rule_set::de_authorization_rule_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_authorization_rules(var_1);
            }
            ,
            s if s.matches("nextToken") /* NextToken com.amazonaws.ec2.synthetic#DescribeClientVpnAuthorizationRulesOutput$NextToken */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
