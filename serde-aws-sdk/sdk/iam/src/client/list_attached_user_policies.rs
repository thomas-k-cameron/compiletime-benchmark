// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAttachedUserPolicies`](crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder::set_user_name): <p>The name (friendly name, not ARN) of the user to list attached policies for.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`path_prefix(impl Into<String>)`](crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder::path_prefix) / [`set_path_prefix(Option<String>)`](crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder::set_path_prefix): <p>The path prefix for filtering the results. This parameter is optional. If it is not included, it defaults to a slash (/), listing all policies.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of either a forward slash (/) by itself or a string that must begin and end with forward slashes. In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including most punctuation characters, digits, and upper and lowercased letters.</p>
    ///   - [`marker(impl Into<String>)`](crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder::set_marker): <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    ///   - [`max_items(i32)`](crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder::max_items) / [`set_max_items(Option<i32>)`](crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder::set_max_items): <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>  <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    /// - On success, responds with [`ListAttachedUserPoliciesOutput`](crate::operation::list_attached_user_policies::ListAttachedUserPoliciesOutput) with field(s):
    ///   - [`attached_policies(Option<Vec<AttachedPolicy>>)`](crate::operation::list_attached_user_policies::ListAttachedUserPoliciesOutput::attached_policies): <p>A list of the attached policies.</p>
    ///   - [`is_truncated(bool)`](crate::operation::list_attached_user_policies::ListAttachedUserPoliciesOutput::is_truncated): <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    ///   - [`marker(Option<String>)`](crate::operation::list_attached_user_policies::ListAttachedUserPoliciesOutput::marker): <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    /// - On failure, responds with [`SdkError<ListAttachedUserPoliciesError>`](crate::operation::list_attached_user_policies::ListAttachedUserPoliciesError)
    pub fn list_attached_user_policies(&self) -> crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder{
        crate::operation::list_attached_user_policies::builders::ListAttachedUserPoliciesFluentBuilder::new(self.handle.clone())
    }
}
