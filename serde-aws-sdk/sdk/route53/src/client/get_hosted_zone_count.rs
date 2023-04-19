// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetHostedZoneCount`](crate::operation::get_hosted_zone_count::builders::GetHostedZoneCountFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_hosted_zone_count::builders::GetHostedZoneCountFluentBuilder::send) it.
    /// - On success, responds with [`GetHostedZoneCountOutput`](crate::operation::get_hosted_zone_count::GetHostedZoneCountOutput) with field(s):
    ///   - [`hosted_zone_count(Option<i64>)`](crate::operation::get_hosted_zone_count::GetHostedZoneCountOutput::hosted_zone_count): <p>The total number of public and private hosted zones that are associated with the current Amazon Web Services account.</p>
    /// - On failure, responds with [`SdkError<GetHostedZoneCountError>`](crate::operation::get_hosted_zone_count::GetHostedZoneCountError)
    pub fn get_hosted_zone_count(
        &self,
    ) -> crate::operation::get_hosted_zone_count::builders::GetHostedZoneCountFluentBuilder {
        crate::operation::get_hosted_zone_count::builders::GetHostedZoneCountFluentBuilder::new(
            self.handle.clone(),
        )
    }
}