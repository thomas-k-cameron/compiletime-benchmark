// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_flow_logs_input_input(
    input: &crate::operation::delete_flow_logs::DeleteFlowLogsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DeleteFlowLogs", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("FlowLogId");
    if let Some(var_4) = &input.flow_log_ids {
        let mut list_6 = scope_3.start_list(true, Some("item"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
