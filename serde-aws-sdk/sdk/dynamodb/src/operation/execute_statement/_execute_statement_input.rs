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
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ExecuteStatementInput {
    /// <p>The PartiQL statement representing the operation to run.</p>
    #[doc(hidden)]
    pub statement: std::option::Option<std::string::String>,
    /// <p>The parameters for the PartiQL statement, if any.</p>
    #[doc(hidden)]
    pub parameters: std::option::Option<std::vec::Vec<crate::types::AttributeValue>>,
    /// <p>The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p>
    #[doc(hidden)]
    pub consistent_read: std::option::Option<bool>,
    /// <p>Set this value to get remaining results, if <code>NextToken</code> was returned in the statement response.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub return_consumed_capacity: std::option::Option<crate::types::ReturnConsumedCapacity>,
    /// <p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, along with a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation so you can pick up where you left off. Also, if the processed dataset size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation to continue the operation. </p>
    #[doc(hidden)]
    pub limit: std::option::Option<i32>,
}
impl ExecuteStatementInput {
    /// <p>The PartiQL statement representing the operation to run.</p>
    pub fn statement(&self) -> std::option::Option<&str> {
        self.statement.as_deref()
    }
    /// <p>The parameters for the PartiQL statement, if any.</p>
    pub fn parameters(&self) -> std::option::Option<&[crate::types::AttributeValue]> {
        self.parameters.as_deref()
    }
    /// <p>The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p>
    pub fn consistent_read(&self) -> std::option::Option<bool> {
        self.consistent_read
    }
    /// <p>Set this value to get remaining results, if <code>NextToken</code> was returned in the statement response.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    pub fn return_consumed_capacity(
        &self,
    ) -> std::option::Option<&crate::types::ReturnConsumedCapacity> {
        self.return_consumed_capacity.as_ref()
    }
    /// <p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, along with a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation so you can pick up where you left off. Also, if the processed dataset size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation to continue the operation. </p>
    pub fn limit(&self) -> std::option::Option<i32> {
        self.limit
    }
}
impl ExecuteStatementInput {
    /// Creates a new builder-style object to manufacture [`ExecuteStatementInput`](crate::operation::execute_statement::ExecuteStatementInput).
    pub fn builder() -> crate::operation::execute_statement::builders::ExecuteStatementInputBuilder
    {
        crate::operation::execute_statement::builders::ExecuteStatementInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::execute_statement::ExecuteStatementInput;
/// A builder for [`ExecuteStatementInput`](crate::operation::execute_statement::ExecuteStatementInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct ExecuteStatementInputBuilder {
    pub(crate) statement: std::option::Option<std::string::String>,
    pub(crate) parameters: std::option::Option<std::vec::Vec<crate::types::AttributeValue>>,
    pub(crate) consistent_read: std::option::Option<bool>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) return_consumed_capacity: std::option::Option<crate::types::ReturnConsumedCapacity>,
    pub(crate) limit: std::option::Option<i32>,
}
impl ExecuteStatementInputBuilder {
    /// <p>The PartiQL statement representing the operation to run.</p>
    pub fn statement(mut self, input: impl Into<std::string::String>) -> Self {
        self.statement = Some(input.into());
        self
    }
    /// <p>The PartiQL statement representing the operation to run.</p>
    pub fn set_statement(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.statement = input;
        self
    }
    /// Appends an item to `parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>The parameters for the PartiQL statement, if any.</p>
    pub fn parameters(mut self, input: crate::types::AttributeValue) -> Self {
        let mut v = self.parameters.unwrap_or_default();
        v.push(input);
        self.parameters = Some(v);
        self
    }
    /// <p>The parameters for the PartiQL statement, if any.</p>
    pub fn set_parameters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AttributeValue>>,
    ) -> Self {
        self.parameters = input;
        self
    }
    /// <p>The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p>
    pub fn consistent_read(mut self, input: bool) -> Self {
        self.consistent_read = Some(input);
        self
    }
    /// <p>The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p>
    pub fn set_consistent_read(mut self, input: std::option::Option<bool>) -> Self {
        self.consistent_read = input;
        self
    }
    /// <p>Set this value to get remaining results, if <code>NextToken</code> was returned in the statement response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>Set this value to get remaining results, if <code>NextToken</code> was returned in the statement response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    pub fn return_consumed_capacity(mut self, input: crate::types::ReturnConsumedCapacity) -> Self {
        self.return_consumed_capacity = Some(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    pub fn set_return_consumed_capacity(
        mut self,
        input: std::option::Option<crate::types::ReturnConsumedCapacity>,
    ) -> Self {
        self.return_consumed_capacity = input;
        self
    }
    /// <p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, along with a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation so you can pick up where you left off. Also, if the processed dataset size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation to continue the operation. </p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = Some(input);
        self
    }
    /// <p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, along with a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation so you can pick up where you left off. Also, if the processed dataset size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation to continue the operation. </p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// Consumes the builder and constructs a [`ExecuteStatementInput`](crate::operation::execute_statement::ExecuteStatementInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::execute_statement::ExecuteStatementInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::execute_statement::ExecuteStatementInput {
            statement: self.statement,
            parameters: self.parameters,
            consistent_read: self.consistent_read,
            next_token: self.next_token,
            return_consumed_capacity: self.return_consumed_capacity,
            limit: self.limit,
        })
    }
}
