// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ReplicateKey`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_key_id): <p>Identifies the multi-Region primary key that is being replicated. To determine whether a KMS key is a multi-Region primary key, use the <code>DescribeKey</code> operation to check the value of the <code>MultiRegionKeyType</code> property.</p>  <p>Specify the key ID or key ARN of a multi-Region primary key.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    ///   - [`replica_region(impl Into<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::replica_region) / [`set_replica_region(Option<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_replica_region): <p>The Region ID of the Amazon Web Services Region for this replica key. </p>  <p>Enter the Region ID, such as <code>us-east-1</code> or <code>ap-southeast-2</code>. For a list of Amazon Web Services Regions in which KMS is supported, see <a href="https://docs.aws.amazon.com/general/latest/gr/kms.html#kms_region">KMS service endpoints</a> in the <i>Amazon Web Services General Reference</i>.</p> <note>   <p>HMAC KMS keys are not supported in all Amazon Web Services Regions. If you try to replicate an HMAC KMS key in an Amazon Web Services Region in which HMAC keys are not supported, the <code>ReplicateKey</code> operation returns an <code>UnsupportedOperationException</code>. For a list of Regions in which HMAC KMS keys are supported, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC keys in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>  </note>  <p>The replica must be in a different Amazon Web Services Region than its primary key and other replicas of that primary key, but in the same Amazon Web Services partition. KMS must be available in the replica Region. If the Region is not enabled by default, the Amazon Web Services account must be enabled in the Region. For information about Amazon Web Services partitions, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. For information about enabling and disabling Regions, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-enable">Enabling a Region</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-disable">Disabling a Region</a> in the <i>Amazon Web Services General Reference</i>.</p>
    ///   - [`policy(impl Into<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_policy): <p>The key policy to attach to the KMS key. This parameter is optional. If you do not provide a key policy, KMS attaches the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default">default key policy</a> to the KMS key.</p>  <p>The key policy is not a shared property of multi-Region keys. You can specify the same key policy or a different key policy for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p>  <p>If you provide a key policy, it must meet the following criteria:</p>  <ul>   <li> <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p> </li>   <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p> </li>  </ul>  <p>A key policy document can include only the following characters:</p>  <ul>   <li> <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p> </li>   <li> <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p> </li>   <li> <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p> </li>  </ul>  <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the <i>Key Management Service Developer Guide</i>. For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i> </i>.</p>
    ///   - [`bypass_policy_lockout_safety_check(bool)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<bool>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_bypass_policy_lockout_safety_check): <p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p> <important>   <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p>   <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p>  </important>  <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <code>PutKeyPolicy</code> request on the KMS key.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_description): <p>A description of the KMS key. The default value is an empty string (no description).</p>  <p>The description is not a shared property of multi-Region keys. You can specify the same description or a different description for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_tags): <p>Assigns one or more tags to the replica key. Use this parameter to tag the KMS key when it is created. To tag an existing KMS key, use the <code>TagResource</code> operation.</p> <note>   <p>Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">ABAC for KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>  </note>  <p>To use this parameter, you must have <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> permission in an IAM policy.</p>  <p>Tags are not a shared property of multi-Region keys. You can specify the same tags or different tags for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p>  <p>Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string. You cannot have more than one tag on a KMS key with the same tag key. If you specify an existing tag key with a different tag value, KMS replaces the current tag value with the specified one.</p>  <p>When you add tags to an Amazon Web Services resource, Amazon Web Services generates a cost allocation report with usage and costs aggregated by tags. Tags can also be used to control access to a KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging Keys</a>.</p>
    /// - On success, responds with [`ReplicateKeyOutput`](crate::operation::replicate_key::ReplicateKeyOutput) with field(s):
    ///   - [`replica_key_metadata(Option<KeyMetadata>)`](crate::operation::replicate_key::ReplicateKeyOutput::replica_key_metadata): <p>Displays details about the new replica key, including its Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a>. It also includes the ARN and Amazon Web Services Region of its primary key and other replica keys.</p>
    ///   - [`replica_policy(Option<String>)`](crate::operation::replicate_key::ReplicateKeyOutput::replica_policy): <p>The key policy of the new replica key. The value is a key policy document in JSON format.</p>
    ///   - [`replica_tags(Option<Vec<Tag>>)`](crate::operation::replicate_key::ReplicateKeyOutput::replica_tags): <p>The tags on the new replica key. The value is a list of tag key and tag value pairs.</p>
    /// - On failure, responds with [`SdkError<ReplicateKeyError>`](crate::operation::replicate_key::ReplicateKeyError)
    pub fn replicate_key(
        &self,
    ) -> crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder {
        crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
