// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartNetworkInsightsAnalysis`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`network_insights_path_id(impl Into<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::network_insights_path_id) / [`set_network_insights_path_id(Option<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_network_insights_path_id): <p>The ID of the path.</p>
    ///   - [`additional_accounts(Vec<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::additional_accounts) / [`set_additional_accounts(Option<Vec<String>>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_additional_accounts): <p>The member accounts that contain resources that the path can traverse.</p>
    ///   - [`filter_in_arns(Vec<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::filter_in_arns) / [`set_filter_in_arns(Option<Vec<String>>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_filter_in_arns): <p>The Amazon Resource Names (ARN) of the resources that the path must traverse.</p>
    ///   - [`dry_run(bool)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_tag_specifications): <p>The tags to apply.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    /// - On success, responds with [`StartNetworkInsightsAnalysisOutput`](crate::operation::start_network_insights_analysis::StartNetworkInsightsAnalysisOutput) with field(s):
    ///   - [`network_insights_analysis(Option<NetworkInsightsAnalysis>)`](crate::operation::start_network_insights_analysis::StartNetworkInsightsAnalysisOutput::network_insights_analysis): <p>Information about the network insights analysis.</p>
    /// - On failure, responds with [`SdkError<StartNetworkInsightsAnalysisError>`](crate::operation::start_network_insights_analysis::StartNetworkInsightsAnalysisError)
    pub fn start_network_insights_analysis(&self) -> crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder{
        crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::new(self.handle.clone())
    }
}