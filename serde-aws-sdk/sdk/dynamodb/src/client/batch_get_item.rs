// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetItem`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`request_items(HashMap<String, KeysAndAttributes>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::request_items) / [`set_request_items(Option<HashMap<String, KeysAndAttributes>>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::set_request_items): <p>A map of one or more table names and, for each table, a map that describes one or more items to retrieve from that table. Each table name can be used only once per <code>BatchGetItem</code> request.</p>  <p>Each element in the map of items to retrieve consists of the following:</p>  <ul>   <li> <p> <code>ConsistentRead</code> - If <code>true</code>, a strongly consistent read is used; if <code>false</code> (the default), an eventually consistent read is used.</p> </li>   <li> <p> <code>ExpressionAttributeNames</code> - One or more substitution tokens for attribute names in the <code>ProjectionExpression</code> parameter. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>    <ul>     <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li>     <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li>     <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li>    </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>    <ul>     <li> <p> <code>Percentile</code> </p> </li>    </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>    <ul>     <li> <p> <code>{"#P":"Percentile"}</code> </p> </li>    </ul> <p>You could then use this substitution in an expression, as in this example:</p>    <ul>     <li> <p> <code>#P = :val</code> </p> </li>    </ul> <note>     <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>    </note> <p>For more information about expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </li>   <li> <p> <code>Keys</code> - An array of primary key attribute values that define specific items in the table. For each primary key, you must provide <i>all</i> of the key attributes. For example, with a simple primary key, you only need to provide the partition key value. For a composite key, you must provide <i>both</i> the partition key value and the sort key value.</p> </li>   <li> <p> <code>ProjectionExpression</code> - A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes are returned. If any of the requested attributes are not found, they do not appear in the result.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </li>   <li> <p> <code>AttributesToGet</code> - This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p> </li>  </ul>
    ///   - [`return_consumed_capacity(ReturnConsumedCapacity)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<ReturnConsumedCapacity>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::set_return_consumed_capacity): <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>  <ul>   <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>   <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>   <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>  </ul>
    /// - On success, responds with [`BatchGetItemOutput`](crate::operation::batch_get_item::BatchGetItemOutput) with field(s):
    ///   - [`responses(Option<HashMap<String, Vec<HashMap<String, AttributeValue>>>>)`](crate::operation::batch_get_item::BatchGetItemOutput::responses): <p>A map of table name to a list of items. Each object in <code>Responses</code> consists of a table name, along with a map of attribute data consisting of the data type and attribute value.</p>
    ///   - [`unprocessed_keys(Option<HashMap<String, KeysAndAttributes>>)`](crate::operation::batch_get_item::BatchGetItemOutput::unprocessed_keys): <p>A map of tables and their respective keys that were not processed with the current response. The <code>UnprocessedKeys</code> value is in the same form as <code>RequestItems</code>, so the value can be provided directly to a subsequent <code>BatchGetItem</code> operation. For more information, see <code>RequestItems</code> in the Request Parameters section.</p>  <p>Each element consists of:</p>  <ul>   <li> <p> <code>Keys</code> - An array of primary key attribute values that define specific items in the table.</p> </li>   <li> <p> <code>ProjectionExpression</code> - One or more attributes to be retrieved from the table or index. By default, all attributes are returned. If a requested attribute is not found, it does not appear in the result.</p> </li>   <li> <p> <code>ConsistentRead</code> - The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p> </li>  </ul>  <p>If there are no unprocessed keys remaining, the response contains an empty <code>UnprocessedKeys</code> map.</p>
    ///   - [`consumed_capacity(Option<Vec<ConsumedCapacity>>)`](crate::operation::batch_get_item::BatchGetItemOutput::consumed_capacity): <p>The read capacity units consumed by the entire <code>BatchGetItem</code> operation.</p>  <p>Each element consists of:</p>  <ul>   <li> <p> <code>TableName</code> - The table that consumed the provisioned throughput.</p> </li>   <li> <p> <code>CapacityUnits</code> - The total number of capacity units consumed.</p> </li>  </ul>
    /// - On failure, responds with [`SdkError<BatchGetItemError>`](crate::operation::batch_get_item::BatchGetItemError)
    pub fn batch_get_item(
        &self,
    ) -> crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder {
        crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
