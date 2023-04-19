// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetManagedPrefixListEntries`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`prefix_list_id(impl Into<String>)`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::prefix_list_id) / [`set_prefix_list_id(Option<String>)`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::set_prefix_list_id): <p>The ID of the prefix list.</p>
    ///   - [`target_version(i64)`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::target_version) / [`set_target_version(Option<i64>)`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::set_target_version): <p>The version of the prefix list for which to return the entries. The default is the current version.</p>
    ///   - [`max_results(i32)`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    /// - On success, responds with [`GetManagedPrefixListEntriesOutput`](crate::operation::get_managed_prefix_list_entries::GetManagedPrefixListEntriesOutput) with field(s):
    ///   - [`entries(Option<Vec<PrefixListEntry>>)`](crate::operation::get_managed_prefix_list_entries::GetManagedPrefixListEntriesOutput::entries): <p>Information about the prefix list entries.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_managed_prefix_list_entries::GetManagedPrefixListEntriesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<GetManagedPrefixListEntriesError>`](crate::operation::get_managed_prefix_list_entries::GetManagedPrefixListEntriesError)
    pub fn get_managed_prefix_list_entries(&self) -> crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder{
        crate::operation::get_managed_prefix_list_entries::builders::GetManagedPrefixListEntriesFluentBuilder::new(self.handle.clone())
    }
}
