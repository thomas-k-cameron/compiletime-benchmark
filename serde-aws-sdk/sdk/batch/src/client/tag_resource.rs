// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TagResource`](crate::operation::tag_resource::builders::TagResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_resource_arn): <p>The Amazon Resource Name (ARN) of the resource that tags are added to. Batch resources that support tags are compute environments, jobs, job definitions, job queues, and scheduling policies. ARNs for child jobs of array and multi-node parallel (MNP) jobs aren't supported.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_tags): <p>The tags that you apply to the resource to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> in <i>Amazon Web Services General Reference</i>.</p>
    /// - On success, responds with [`TagResourceOutput`](crate::operation::tag_resource::TagResourceOutput)
    /// - On failure, responds with [`SdkError<TagResourceError>`](crate::operation::tag_resource::TagResourceError)
    pub fn tag_resource(
        &self,
    ) -> crate::operation::tag_resource::builders::TagResourceFluentBuilder {
        crate::operation::tag_resource::builders::TagResourceFluentBuilder::new(self.handle.clone())
    }
}