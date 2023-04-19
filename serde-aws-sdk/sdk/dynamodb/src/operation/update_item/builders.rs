// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_item::_update_item_output::UpdateItemOutputBuilder;

pub use crate::operation::update_item::_update_item_input::UpdateItemInputBuilder;

/// Fluent builder constructing a request to `UpdateItem`.
///
/// <p>Edits an existing item's attributes, or adds a new item to the table if it does not already exist. You can put, delete, or add attribute values. You can also perform a conditional update on an existing item (insert a new attribute name-value pair if it doesn't exist, or replace an existing name-value pair if it has certain expected attribute values).</p>
/// <p>You can also return the item's attribute values in the same <code>UpdateItem</code> operation using the <code>ReturnValues</code> parameter.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateItemFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_item::builders::UpdateItemInputBuilder,
}
impl UpdateItemFluentBuilder {
    /// Creates a new `UpdateItem`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_item::UpdateItem,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_item::UpdateItemError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::update_item::UpdateItemOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_item::UpdateItemError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::update_item::builders::UpdateItemInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.update_item().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::update_item::builders::UpdateItemInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the table containing the item to update.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table containing the item to update.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// Adds a key-value pair to `Key`.
    ///
    /// To override the contents of this collection use [`set_key`](Self::set_key).
    ///
    /// <p>The primary key of the item to be updated. Each element consists of an attribute name and a value for that attribute.</p>
    /// <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    pub fn key(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::AttributeValue,
    ) -> Self {
        self.inner = self.inner.key(k.into(), v);
        self
    }
    /// <p>The primary key of the item to be updated. Each element consists of an attribute name and a value for that attribute.</p>
    /// <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    pub fn set_key(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
        >,
    ) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    /// Adds a key-value pair to `AttributeUpdates`.
    ///
    /// To override the contents of this collection use [`set_attribute_updates`](Self::set_attribute_updates).
    ///
    /// <p>This is a legacy parameter. Use <code>UpdateExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributeUpdates.html">AttributeUpdates</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn attribute_updates(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::AttributeValueUpdate,
    ) -> Self {
        self.inner = self.inner.attribute_updates(k.into(), v);
        self
    }
    /// <p>This is a legacy parameter. Use <code>UpdateExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributeUpdates.html">AttributeUpdates</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_attribute_updates(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::AttributeValueUpdate>,
        >,
    ) -> Self {
        self.inner = self.inner.set_attribute_updates(input);
        self
    }
    /// Adds a key-value pair to `Expected`.
    ///
    /// To override the contents of this collection use [`set_expected`](Self::set_expected).
    ///
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expected(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::ExpectedAttributeValue,
    ) -> Self {
        self.inner = self.inner.expected(k.into(), v);
        self
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expected(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::ExpectedAttributeValue>,
        >,
    ) -> Self {
        self.inner = self.inner.set_expected(input);
        self
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn conditional_operator(mut self, input: crate::types::ConditionalOperator) -> Self {
        self.inner = self.inner.conditional_operator(input);
        self
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_conditional_operator(
        mut self,
        input: std::option::Option<crate::types::ConditionalOperator>,
    ) -> Self {
        self.inner = self.inner.set_conditional_operator(input);
        self
    }
    /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appear before or after they are successfully updated. For <code>UpdateItem</code>, the valid values are:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p> </li>
    /// <li> <p> <code>ALL_OLD</code> - Returns all of the attributes of the item, as they appeared before the UpdateItem operation.</p> </li>
    /// <li> <p> <code>UPDATED_OLD</code> - Returns only the updated attributes, as they appeared before the UpdateItem operation.</p> </li>
    /// <li> <p> <code>ALL_NEW</code> - Returns all of the attributes of the item, as they appear after the UpdateItem operation.</p> </li>
    /// <li> <p> <code>UPDATED_NEW</code> - Returns only the updated attributes, as they appear after the UpdateItem operation.</p> </li>
    /// </ul>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    /// <p>The values returned are strongly consistent.</p>
    pub fn return_values(mut self, input: crate::types::ReturnValue) -> Self {
        self.inner = self.inner.return_values(input);
        self
    }
    /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appear before or after they are successfully updated. For <code>UpdateItem</code>, the valid values are:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p> </li>
    /// <li> <p> <code>ALL_OLD</code> - Returns all of the attributes of the item, as they appeared before the UpdateItem operation.</p> </li>
    /// <li> <p> <code>UPDATED_OLD</code> - Returns only the updated attributes, as they appeared before the UpdateItem operation.</p> </li>
    /// <li> <p> <code>ALL_NEW</code> - Returns all of the attributes of the item, as they appear after the UpdateItem operation.</p> </li>
    /// <li> <p> <code>UPDATED_NEW</code> - Returns only the updated attributes, as they appear after the UpdateItem operation.</p> </li>
    /// </ul>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    /// <p>The values returned are strongly consistent.</p>
    pub fn set_return_values(
        mut self,
        input: std::option::Option<crate::types::ReturnValue>,
    ) -> Self {
        self.inner = self.inner.set_return_values(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    pub fn return_consumed_capacity(mut self, input: crate::types::ReturnConsumedCapacity) -> Self {
        self.inner = self.inner.return_consumed_capacity(input);
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
        self.inner = self.inner.set_return_consumed_capacity(input);
        self
    }
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    pub fn return_item_collection_metrics(
        mut self,
        input: crate::types::ReturnItemCollectionMetrics,
    ) -> Self {
        self.inner = self.inner.return_item_collection_metrics(input);
        self
    }
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    pub fn set_return_item_collection_metrics(
        mut self,
        input: std::option::Option<crate::types::ReturnItemCollectionMetrics>,
    ) -> Self {
        self.inner = self.inner.set_return_item_collection_metrics(input);
        self
    }
    /// <p>An expression that defines one or more attributes to be updated, the action to be performed on them, and new values for them.</p>
    /// <p>The following action values are available for <code>UpdateExpression</code>.</p>
    /// <ul>
    /// <li> <p> <code>SET</code> - Adds one or more attributes and values to an item. If any of these attributes already exist, they are replaced by the new values. You can also use <code>SET</code> to add or subtract from an attribute that is of type Number. For example: <code>SET myNum = myNum + :val</code> </p> <p> <code>SET</code> supports the following functions:</p>
    /// <ul>
    /// <li> <p> <code>if_not_exists (path, operand)</code> - if the item does not contain an attribute at the specified path, then <code>if_not_exists</code> evaluates to operand; otherwise, it evaluates to path. You can use this function to avoid overwriting an attribute that may already be present in the item.</p> </li>
    /// <li> <p> <code>list_append (operand, operand)</code> - evaluates to a list with a new element added to it. You can append the new element to the start or the end of the list by reversing the order of the operands.</p> </li>
    /// </ul> <p>These function names are case-sensitive.</p> </li>
    /// <li> <p> <code>REMOVE</code> - Removes one or more attributes from an item.</p> </li>
    /// <li> <p> <code>ADD</code> - Adds the specified value to the item, if the attribute does not already exist. If the attribute does exist, then the behavior of <code>ADD</code> depends on the data type of the attribute:</p>
    /// <ul>
    /// <li> <p>If the existing attribute is a number, and if <code>Value</code> is also a number, then <code>Value</code> is mathematically added to the existing attribute. If <code>Value</code> is a negative number, then it is subtracted from the existing attribute.</p> <note>
    /// <p>If you use <code>ADD</code> to increment or decrement a number value for an item that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value.</p>
    /// <p>Similarly, if you use <code>ADD</code> for an existing item to increment or decrement an attribute value that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value. For example, suppose that the item you want to update doesn't have an attribute named <code>itemcount</code>, but you decide to <code>ADD</code> the number <code>3</code> to this attribute anyway. DynamoDB will create the <code>itemcount</code> attribute, set its initial value to <code>0</code>, and finally add <code>3</code> to it. The result will be a new <code>itemcount</code> attribute in the item, with a value of <code>3</code>.</p>
    /// </note> </li>
    /// <li> <p>If the existing data type is a set and if <code>Value</code> is also a set, then <code>Value</code> is added to the existing set. For example, if the attribute value is the set <code>[1,2]</code>, and the <code>ADD</code> action specified <code>[3]</code>, then the final attribute value is <code>[1,2,3]</code>. An error occurs if an <code>ADD</code> action is specified for a set attribute and the attribute type specified does not match the existing set type. </p> <p>Both sets must have the same primitive data type. For example, if the existing data type is a set of strings, the <code>Value</code> must also be a set of strings.</p> </li>
    /// </ul> <important>
    /// <p>The <code>ADD</code> action only supports Number and set data types. In addition, <code>ADD</code> can only be used on top-level attributes, not nested attributes.</p>
    /// </important> </li>
    /// <li> <p> <code>DELETE</code> - Deletes an element from a set.</p> <p>If a set of values is specified, then those values are subtracted from the old set. For example, if the attribute value was the set <code>[a,b,c]</code> and the <code>DELETE</code> action specifies <code>[a,c]</code>, then the final attribute value is <code>[b]</code>. Specifying an empty set is an error.</p> <important>
    /// <p>The <code>DELETE</code> action only supports set data types. In addition, <code>DELETE</code> can only be used on top-level attributes, not nested attributes.</p>
    /// </important> </li>
    /// </ul>
    /// <p>You can have many actions in a single expression, such as the following: <code>SET a=:value1, b=:value2 DELETE :value3, :value4, :value5</code> </p>
    /// <p>For more information on update expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Modifying.html">Modifying Items and Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn update_expression(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.update_expression(input.into());
        self
    }
    /// <p>An expression that defines one or more attributes to be updated, the action to be performed on them, and new values for them.</p>
    /// <p>The following action values are available for <code>UpdateExpression</code>.</p>
    /// <ul>
    /// <li> <p> <code>SET</code> - Adds one or more attributes and values to an item. If any of these attributes already exist, they are replaced by the new values. You can also use <code>SET</code> to add or subtract from an attribute that is of type Number. For example: <code>SET myNum = myNum + :val</code> </p> <p> <code>SET</code> supports the following functions:</p>
    /// <ul>
    /// <li> <p> <code>if_not_exists (path, operand)</code> - if the item does not contain an attribute at the specified path, then <code>if_not_exists</code> evaluates to operand; otherwise, it evaluates to path. You can use this function to avoid overwriting an attribute that may already be present in the item.</p> </li>
    /// <li> <p> <code>list_append (operand, operand)</code> - evaluates to a list with a new element added to it. You can append the new element to the start or the end of the list by reversing the order of the operands.</p> </li>
    /// </ul> <p>These function names are case-sensitive.</p> </li>
    /// <li> <p> <code>REMOVE</code> - Removes one or more attributes from an item.</p> </li>
    /// <li> <p> <code>ADD</code> - Adds the specified value to the item, if the attribute does not already exist. If the attribute does exist, then the behavior of <code>ADD</code> depends on the data type of the attribute:</p>
    /// <ul>
    /// <li> <p>If the existing attribute is a number, and if <code>Value</code> is also a number, then <code>Value</code> is mathematically added to the existing attribute. If <code>Value</code> is a negative number, then it is subtracted from the existing attribute.</p> <note>
    /// <p>If you use <code>ADD</code> to increment or decrement a number value for an item that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value.</p>
    /// <p>Similarly, if you use <code>ADD</code> for an existing item to increment or decrement an attribute value that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value. For example, suppose that the item you want to update doesn't have an attribute named <code>itemcount</code>, but you decide to <code>ADD</code> the number <code>3</code> to this attribute anyway. DynamoDB will create the <code>itemcount</code> attribute, set its initial value to <code>0</code>, and finally add <code>3</code> to it. The result will be a new <code>itemcount</code> attribute in the item, with a value of <code>3</code>.</p>
    /// </note> </li>
    /// <li> <p>If the existing data type is a set and if <code>Value</code> is also a set, then <code>Value</code> is added to the existing set. For example, if the attribute value is the set <code>[1,2]</code>, and the <code>ADD</code> action specified <code>[3]</code>, then the final attribute value is <code>[1,2,3]</code>. An error occurs if an <code>ADD</code> action is specified for a set attribute and the attribute type specified does not match the existing set type. </p> <p>Both sets must have the same primitive data type. For example, if the existing data type is a set of strings, the <code>Value</code> must also be a set of strings.</p> </li>
    /// </ul> <important>
    /// <p>The <code>ADD</code> action only supports Number and set data types. In addition, <code>ADD</code> can only be used on top-level attributes, not nested attributes.</p>
    /// </important> </li>
    /// <li> <p> <code>DELETE</code> - Deletes an element from a set.</p> <p>If a set of values is specified, then those values are subtracted from the old set. For example, if the attribute value was the set <code>[a,b,c]</code> and the <code>DELETE</code> action specifies <code>[a,c]</code>, then the final attribute value is <code>[b]</code>. Specifying an empty set is an error.</p> <important>
    /// <p>The <code>DELETE</code> action only supports set data types. In addition, <code>DELETE</code> can only be used on top-level attributes, not nested attributes.</p>
    /// </important> </li>
    /// </ul>
    /// <p>You can have many actions in a single expression, such as the following: <code>SET a=:value1, b=:value2 DELETE :value3, :value4, :value5</code> </p>
    /// <p>For more information on update expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Modifying.html">Modifying Items and Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_update_expression(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_update_expression(input);
        self
    }
    /// <p>A condition that must be satisfied in order for a conditional update to succeed.</p>
    /// <p>An expression can contain any of the following:</p>
    /// <ul>
    /// <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li>
    /// <li> <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code> </p> </li>
    /// <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li>
    /// </ul>
    /// <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn condition_expression(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.condition_expression(input.into());
        self
    }
    /// <p>A condition that must be satisfied in order for a conditional update to succeed.</p>
    /// <p>An expression can contain any of the following:</p>
    /// <ul>
    /// <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li>
    /// <li> <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code> </p> </li>
    /// <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li>
    /// </ul>
    /// <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_condition_expression(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_condition_expression(input);
        self
    }
    /// Adds a key-value pair to `ExpressionAttributeNames`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_names`](Self::set_expression_attribute_names).
    ///
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li>
    /// <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li>
    /// <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li>
    /// </ul>
    /// <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>
    /// <ul>
    /// <li> <p> <code>Percentile</code> </p> </li>
    /// </ul>
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>.) To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p> <code>{"#P":"Percentile"}</code> </p> </li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li> <p> <code>#P = :val</code> </p> </li>
    /// </ul> <note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information about expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_names(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.expression_attribute_names(k.into(), v.into());
        self
    }
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li>
    /// <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li>
    /// <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li>
    /// </ul>
    /// <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>
    /// <ul>
    /// <li> <p> <code>Percentile</code> </p> </li>
    /// </ul>
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>.) To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p> <code>{"#P":"Percentile"}</code> </p> </li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li> <p> <code>#P = :val</code> </p> </li>
    /// </ul> <note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information about expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expression_attribute_names(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_expression_attribute_names(input);
        self
    }
    /// Adds a key-value pair to `ExpressionAttributeValues`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
    ///
    /// <p>One or more values that can be substituted in an expression.</p>
    /// <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <code>ProductStatus</code> attribute was one of the following: </p>
    /// <p> <code>Available | Backordered | Discontinued</code> </p>
    /// <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p>
    /// <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p>
    /// <p>You could then use these values in an expression, such as this:</p>
    /// <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p>
    /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_values(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::AttributeValue,
    ) -> Self {
        self.inner = self.inner.expression_attribute_values(k.into(), v);
        self
    }
    /// <p>One or more values that can be substituted in an expression.</p>
    /// <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <code>ProductStatus</code> attribute was one of the following: </p>
    /// <p> <code>Available | Backordered | Discontinued</code> </p>
    /// <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p>
    /// <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p>
    /// <p>You could then use these values in an expression, such as this:</p>
    /// <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p>
    /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expression_attribute_values(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
        >,
    ) -> Self {
        self.inner = self.inner.set_expression_attribute_values(input);
        self
    }
}
