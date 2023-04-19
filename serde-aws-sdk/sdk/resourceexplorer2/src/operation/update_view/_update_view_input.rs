// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateViewInput {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that you want to modify.</p>
    #[doc(hidden)]
    pub view_arn: std::option::Option<std::string::String>,
    /// <p>Specifies optional fields that you want included in search results from this view. It is a list of objects that each describe a field to include.</p>
    /// <p>The default is an empty list, with no optional fields included in the results.</p>
    #[doc(hidden)]
    pub included_properties: std::option::Option<std::vec::Vec<crate::types::IncludedProperty>>,
    /// <p>An array of strings that specify which resources are included in the results of queries made using this view. When you use this view in a <code>Search</code> operation, the filter string is combined with the search's <code>QueryString</code> parameter using a logical <code>AND</code> operator.</p>
    /// <p>For information about the supported syntax, see <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html">Search query reference for Resource Explorer</a> in the <i>Amazon Web Services Resource Explorer User Guide</i>.</p> <important>
    /// <p>This query string in the context of this operation supports only <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-filters">filter prefixes</a> with optional <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-operators">operators</a>. It doesn't support free-form text. For example, the string <code>region:us* service:ec2 -tag:stage=prod</code> includes all Amazon EC2 resources in any Amazon Web Services Region that begins with the letters <code>us</code> and is <i>not</i> tagged with a key <code>Stage</code> that has the value <code>prod</code>.</p>
    /// </important>
    #[doc(hidden)]
    pub filters: std::option::Option<crate::types::SearchFilter>,
}
impl UpdateViewInput {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that you want to modify.</p>
    pub fn view_arn(&self) -> std::option::Option<&str> {
        self.view_arn.as_deref()
    }
    /// <p>Specifies optional fields that you want included in search results from this view. It is a list of objects that each describe a field to include.</p>
    /// <p>The default is an empty list, with no optional fields included in the results.</p>
    pub fn included_properties(&self) -> std::option::Option<&[crate::types::IncludedProperty]> {
        self.included_properties.as_deref()
    }
    /// <p>An array of strings that specify which resources are included in the results of queries made using this view. When you use this view in a <code>Search</code> operation, the filter string is combined with the search's <code>QueryString</code> parameter using a logical <code>AND</code> operator.</p>
    /// <p>For information about the supported syntax, see <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html">Search query reference for Resource Explorer</a> in the <i>Amazon Web Services Resource Explorer User Guide</i>.</p> <important>
    /// <p>This query string in the context of this operation supports only <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-filters">filter prefixes</a> with optional <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-operators">operators</a>. It doesn't support free-form text. For example, the string <code>region:us* service:ec2 -tag:stage=prod</code> includes all Amazon EC2 resources in any Amazon Web Services Region that begins with the letters <code>us</code> and is <i>not</i> tagged with a key <code>Stage</code> that has the value <code>prod</code>.</p>
    /// </important>
    pub fn filters(&self) -> std::option::Option<&crate::types::SearchFilter> {
        self.filters.as_ref()
    }
}
impl std::fmt::Debug for UpdateViewInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateViewInput");
        formatter.field("view_arn", &self.view_arn);
        formatter.field("included_properties", &self.included_properties);
        formatter.field("filters", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl UpdateViewInput {
    /// Creates a new builder-style object to manufacture [`UpdateViewInput`](crate::operation::update_view::UpdateViewInput).
    pub fn builder() -> crate::operation::update_view::builders::UpdateViewInputBuilder {
        crate::operation::update_view::builders::UpdateViewInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::update_view::UpdateViewInput;
/// A builder for [`UpdateViewInput`](crate::operation::update_view::UpdateViewInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct UpdateViewInputBuilder {
    pub(crate) view_arn: std::option::Option<std::string::String>,
    pub(crate) included_properties:
        std::option::Option<std::vec::Vec<crate::types::IncludedProperty>>,
    pub(crate) filters: std::option::Option<crate::types::SearchFilter>,
}
impl UpdateViewInputBuilder {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that you want to modify.</p>
    pub fn view_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.view_arn = Some(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that you want to modify.</p>
    pub fn set_view_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.view_arn = input;
        self
    }
    /// Appends an item to `included_properties`.
    ///
    /// To override the contents of this collection use [`set_included_properties`](Self::set_included_properties).
    ///
    /// <p>Specifies optional fields that you want included in search results from this view. It is a list of objects that each describe a field to include.</p>
    /// <p>The default is an empty list, with no optional fields included in the results.</p>
    pub fn included_properties(mut self, input: crate::types::IncludedProperty) -> Self {
        let mut v = self.included_properties.unwrap_or_default();
        v.push(input);
        self.included_properties = Some(v);
        self
    }
    /// <p>Specifies optional fields that you want included in search results from this view. It is a list of objects that each describe a field to include.</p>
    /// <p>The default is an empty list, with no optional fields included in the results.</p>
    pub fn set_included_properties(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::IncludedProperty>>,
    ) -> Self {
        self.included_properties = input;
        self
    }
    /// <p>An array of strings that specify which resources are included in the results of queries made using this view. When you use this view in a <code>Search</code> operation, the filter string is combined with the search's <code>QueryString</code> parameter using a logical <code>AND</code> operator.</p>
    /// <p>For information about the supported syntax, see <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html">Search query reference for Resource Explorer</a> in the <i>Amazon Web Services Resource Explorer User Guide</i>.</p> <important>
    /// <p>This query string in the context of this operation supports only <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-filters">filter prefixes</a> with optional <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-operators">operators</a>. It doesn't support free-form text. For example, the string <code>region:us* service:ec2 -tag:stage=prod</code> includes all Amazon EC2 resources in any Amazon Web Services Region that begins with the letters <code>us</code> and is <i>not</i> tagged with a key <code>Stage</code> that has the value <code>prod</code>.</p>
    /// </important>
    pub fn filters(mut self, input: crate::types::SearchFilter) -> Self {
        self.filters = Some(input);
        self
    }
    /// <p>An array of strings that specify which resources are included in the results of queries made using this view. When you use this view in a <code>Search</code> operation, the filter string is combined with the search's <code>QueryString</code> parameter using a logical <code>AND</code> operator.</p>
    /// <p>For information about the supported syntax, see <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html">Search query reference for Resource Explorer</a> in the <i>Amazon Web Services Resource Explorer User Guide</i>.</p> <important>
    /// <p>This query string in the context of this operation supports only <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-filters">filter prefixes</a> with optional <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-operators">operators</a>. It doesn't support free-form text. For example, the string <code>region:us* service:ec2 -tag:stage=prod</code> includes all Amazon EC2 resources in any Amazon Web Services Region that begins with the letters <code>us</code> and is <i>not</i> tagged with a key <code>Stage</code> that has the value <code>prod</code>.</p>
    /// </important>
    pub fn set_filters(mut self, input: std::option::Option<crate::types::SearchFilter>) -> Self {
        self.filters = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateViewInput`](crate::operation::update_view::UpdateViewInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::update_view::UpdateViewInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::update_view::UpdateViewInput {
            view_arn: self.view_arn,
            included_properties: self.included_properties,
            filters: self.filters,
        })
    }
}
impl std::fmt::Debug for UpdateViewInputBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateViewInputBuilder");
        formatter.field("view_arn", &self.view_arn);
        formatter.field("included_properties", &self.included_properties);
        formatter.field("filters", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
