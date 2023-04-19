// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the input of a <code>DeleteItem</code> operation.</p>
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
pub struct DeleteItemInput {
    /// <p>The name of the table from which to delete the item.</p>
    #[doc(hidden)]
    pub table_name: std::option::Option<std::string::String>,
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to delete.</p>
    /// <p>For the primary key, you must provide all of the key attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    #[doc(hidden)]
    pub key: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    >,
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[doc(hidden)]
    pub expected: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::ExpectedAttributeValue>,
    >,
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[doc(hidden)]
    pub conditional_operator: std::option::Option<crate::types::ConditionalOperator>,
    /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were deleted. For <code>DeleteItem</code>, the valid values are:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p> </li>
    /// <li> <p> <code>ALL_OLD</code> - The content of the old item is returned.</p> </li>
    /// </ul>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p> <note>
    /// <p>The <code>ReturnValues</code> parameter is used by several DynamoDB operations; however, <code>DeleteItem</code> does not recognize any values other than <code>NONE</code> or <code>ALL_OLD</code>.</p>
    /// </note>
    #[doc(hidden)]
    pub return_values: std::option::Option<crate::types::ReturnValue>,
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub return_consumed_capacity: std::option::Option<crate::types::ReturnConsumedCapacity>,
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    #[doc(hidden)]
    pub return_item_collection_metrics:
        std::option::Option<crate::types::ReturnItemCollectionMetrics>,
    /// <p>A condition that must be satisfied in order for a conditional <code>DeleteItem</code> to succeed.</p>
    /// <p>An expression can contain any of the following:</p>
    /// <ul>
    /// <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li>
    /// <li> <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code> </p> </li>
    /// <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li>
    /// </ul>
    /// <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[doc(hidden)]
    pub condition_expression: std::option::Option<std::string::String>,
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
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p> <code>{"#P":"Percentile"}</code> </p> </li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li> <p> <code>#P = :val</code> </p> </li>
    /// </ul> <note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[doc(hidden)]
    pub expression_attribute_names:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>One or more values that can be substituted in an expression.</p>
    /// <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p>
    /// <p> <code>Available | Backordered | Discontinued</code> </p>
    /// <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p>
    /// <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p>
    /// <p>You could then use these values in an expression, such as this:</p>
    /// <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p>
    /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[doc(hidden)]
    pub expression_attribute_values: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    >,
}
impl DeleteItemInput {
    /// <p>The name of the table from which to delete the item.</p>
    pub fn table_name(&self) -> std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to delete.</p>
    /// <p>For the primary key, you must provide all of the key attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    pub fn key(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    > {
        self.key.as_ref()
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expected(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<std::string::String, crate::types::ExpectedAttributeValue>,
    > {
        self.expected.as_ref()
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn conditional_operator(&self) -> std::option::Option<&crate::types::ConditionalOperator> {
        self.conditional_operator.as_ref()
    }
    /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were deleted. For <code>DeleteItem</code>, the valid values are:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p> </li>
    /// <li> <p> <code>ALL_OLD</code> - The content of the old item is returned.</p> </li>
    /// </ul>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p> <note>
    /// <p>The <code>ReturnValues</code> parameter is used by several DynamoDB operations; however, <code>DeleteItem</code> does not recognize any values other than <code>NONE</code> or <code>ALL_OLD</code>.</p>
    /// </note>
    pub fn return_values(&self) -> std::option::Option<&crate::types::ReturnValue> {
        self.return_values.as_ref()
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
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    pub fn return_item_collection_metrics(
        &self,
    ) -> std::option::Option<&crate::types::ReturnItemCollectionMetrics> {
        self.return_item_collection_metrics.as_ref()
    }
    /// <p>A condition that must be satisfied in order for a conditional <code>DeleteItem</code> to succeed.</p>
    /// <p>An expression can contain any of the following:</p>
    /// <ul>
    /// <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li>
    /// <li> <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code> </p> </li>
    /// <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li>
    /// </ul>
    /// <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn condition_expression(&self) -> std::option::Option<&str> {
        self.condition_expression.as_deref()
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
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p> <code>{"#P":"Percentile"}</code> </p> </li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li> <p> <code>#P = :val</code> </p> </li>
    /// </ul> <note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_names(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.expression_attribute_names.as_ref()
    }
    /// <p>One or more values that can be substituted in an expression.</p>
    /// <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p>
    /// <p> <code>Available | Backordered | Discontinued</code> </p>
    /// <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p>
    /// <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p>
    /// <p>You could then use these values in an expression, such as this:</p>
    /// <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p>
    /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_values(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    > {
        self.expression_attribute_values.as_ref()
    }
}
impl DeleteItemInput {
    /// Creates a new builder-style object to manufacture [`DeleteItemInput`](crate::operation::delete_item::DeleteItemInput).
    pub fn builder() -> crate::operation::delete_item::builders::DeleteItemInputBuilder {
        crate::operation::delete_item::builders::DeleteItemInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_item::DeleteItemInput;
/// A builder for [`DeleteItemInput`](crate::operation::delete_item::DeleteItemInput).
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
pub struct DeleteItemInputBuilder {
    pub(crate) table_name: std::option::Option<std::string::String>,
    pub(crate) key: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    >,
    pub(crate) expected: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::ExpectedAttributeValue>,
    >,
    pub(crate) conditional_operator: std::option::Option<crate::types::ConditionalOperator>,
    pub(crate) return_values: std::option::Option<crate::types::ReturnValue>,
    pub(crate) return_consumed_capacity: std::option::Option<crate::types::ReturnConsumedCapacity>,
    pub(crate) return_item_collection_metrics:
        std::option::Option<crate::types::ReturnItemCollectionMetrics>,
    pub(crate) condition_expression: std::option::Option<std::string::String>,
    pub(crate) expression_attribute_names:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    pub(crate) expression_attribute_values: std::option::Option<
        std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
    >,
}
impl DeleteItemInputBuilder {
    /// <p>The name of the table from which to delete the item.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.table_name = Some(input.into());
        self
    }
    /// <p>The name of the table from which to delete the item.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// Adds a key-value pair to `key`.
    ///
    /// To override the contents of this collection use [`set_key`](Self::set_key).
    ///
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to delete.</p>
    /// <p>For the primary key, you must provide all of the key attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    pub fn key(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::AttributeValue,
    ) -> Self {
        let mut hash_map = self.key.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.key = Some(hash_map);
        self
    }
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to delete.</p>
    /// <p>For the primary key, you must provide all of the key attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    pub fn set_key(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::AttributeValue>,
        >,
    ) -> Self {
        self.key = input;
        self
    }
    /// Adds a key-value pair to `expected`.
    ///
    /// To override the contents of this collection use [`set_expected`](Self::set_expected).
    ///
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expected(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::ExpectedAttributeValue,
    ) -> Self {
        let mut hash_map = self.expected.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.expected = Some(hash_map);
        self
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expected(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::ExpectedAttributeValue>,
        >,
    ) -> Self {
        self.expected = input;
        self
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn conditional_operator(mut self, input: crate::types::ConditionalOperator) -> Self {
        self.conditional_operator = Some(input);
        self
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_conditional_operator(
        mut self,
        input: std::option::Option<crate::types::ConditionalOperator>,
    ) -> Self {
        self.conditional_operator = input;
        self
    }
    /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were deleted. For <code>DeleteItem</code>, the valid values are:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p> </li>
    /// <li> <p> <code>ALL_OLD</code> - The content of the old item is returned.</p> </li>
    /// </ul>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p> <note>
    /// <p>The <code>ReturnValues</code> parameter is used by several DynamoDB operations; however, <code>DeleteItem</code> does not recognize any values other than <code>NONE</code> or <code>ALL_OLD</code>.</p>
    /// </note>
    pub fn return_values(mut self, input: crate::types::ReturnValue) -> Self {
        self.return_values = Some(input);
        self
    }
    /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were deleted. For <code>DeleteItem</code>, the valid values are:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p> </li>
    /// <li> <p> <code>ALL_OLD</code> - The content of the old item is returned.</p> </li>
    /// </ul>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p> <note>
    /// <p>The <code>ReturnValues</code> parameter is used by several DynamoDB operations; however, <code>DeleteItem</code> does not recognize any values other than <code>NONE</code> or <code>ALL_OLD</code>.</p>
    /// </note>
    pub fn set_return_values(
        mut self,
        input: std::option::Option<crate::types::ReturnValue>,
    ) -> Self {
        self.return_values = input;
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
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    pub fn return_item_collection_metrics(
        mut self,
        input: crate::types::ReturnItemCollectionMetrics,
    ) -> Self {
        self.return_item_collection_metrics = Some(input);
        self
    }
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    pub fn set_return_item_collection_metrics(
        mut self,
        input: std::option::Option<crate::types::ReturnItemCollectionMetrics>,
    ) -> Self {
        self.return_item_collection_metrics = input;
        self
    }
    /// <p>A condition that must be satisfied in order for a conditional <code>DeleteItem</code> to succeed.</p>
    /// <p>An expression can contain any of the following:</p>
    /// <ul>
    /// <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li>
    /// <li> <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code> </p> </li>
    /// <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li>
    /// </ul>
    /// <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn condition_expression(mut self, input: impl Into<std::string::String>) -> Self {
        self.condition_expression = Some(input.into());
        self
    }
    /// <p>A condition that must be satisfied in order for a conditional <code>DeleteItem</code> to succeed.</p>
    /// <p>An expression can contain any of the following:</p>
    /// <ul>
    /// <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li>
    /// <li> <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code> </p> </li>
    /// <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li>
    /// </ul>
    /// <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_condition_expression(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.condition_expression = input;
        self
    }
    /// Adds a key-value pair to `expression_attribute_names`.
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
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p> <code>{"#P":"Percentile"}</code> </p> </li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li> <p> <code>#P = :val</code> </p> </li>
    /// </ul> <note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_names(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        let mut hash_map = self.expression_attribute_names.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.expression_attribute_names = Some(hash_map);
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
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p> <code>{"#P":"Percentile"}</code> </p> </li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li> <p> <code>#P = :val</code> </p> </li>
    /// </ul> <note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expression_attribute_names(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.expression_attribute_names = input;
        self
    }
    /// Adds a key-value pair to `expression_attribute_values`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
    ///
    /// <p>One or more values that can be substituted in an expression.</p>
    /// <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p>
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
        let mut hash_map = self.expression_attribute_values.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.expression_attribute_values = Some(hash_map);
        self
    }
    /// <p>One or more values that can be substituted in an expression.</p>
    /// <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p>
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
        self.expression_attribute_values = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteItemInput`](crate::operation::delete_item::DeleteItemInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_item::DeleteItemInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::delete_item::DeleteItemInput {
            table_name: self.table_name,
            key: self.key,
            expected: self.expected,
            conditional_operator: self.conditional_operator,
            return_values: self.return_values,
            return_consumed_capacity: self.return_consumed_capacity,
            return_item_collection_metrics: self.return_item_collection_metrics,
            condition_expression: self.condition_expression,
            expression_attribute_names: self.expression_attribute_names,
            expression_attribute_values: self.expression_attribute_values,
        })
    }
}
