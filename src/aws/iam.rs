//! Types for the `IAM` service.

/// The [`AWS::IAM::AccessKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html) resource type.
#[derive(Debug)]
pub struct AccessKey {
    properties: AccessKeyProperties
}

/// Properties for the `AccessKey` resource.
#[derive(Debug)]
pub struct AccessKeyProperties {
    /// Property `Serial`.
    pub serial: Option<::Value<u32>>,
    /// Property `Status`.
    pub status: Option<::Value<String>>,
    /// Property `UserName`.
    pub user_name: ::Value<String>,
}

impl ::serde::Serialize for AccessKeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Serial", &self.serial)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", &self.user_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AccessKeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessKeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AccessKeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AccessKeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut serial = None;
                let mut status = None;
                let mut user_name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Serial" => {
                            serial = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Status" => {
                            status = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UserName" => {
                            user_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(AccessKeyProperties {
                    serial: serial,
                    status: status,
                    user_name: user_name.ok_or(::serde::de::Error::missing_field("UserName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AccessKey {
    type Properties = AccessKeyProperties;
    const TYPE: &'static str = "AWS::IAM::AccessKey";
    fn properties(&self) -> &AccessKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AccessKeyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AccessKey {}

impl From<AccessKeyProperties> for AccessKey {
    fn from(properties: AccessKeyProperties) -> AccessKey {
        AccessKey { properties }
    }
}

/// The [`AWS::IAM::Group`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html) resource type.
#[derive(Debug)]
pub struct Group {
    properties: GroupProperties
}

/// Properties for the `Group` resource.
#[derive(Debug)]
pub struct GroupProperties {
    /// Property `GroupName`.
    pub group_name: Option<::Value<String>>,
    /// Property `ManagedPolicyArns`.
    pub managed_policy_arns: Option<::ValueList<String>>,
    /// Property `Path`.
    pub path: Option<::Value<String>>,
    /// Property `Policies`.
    pub policies: Option<::ValueList<self::group::Policy>>,
}

impl ::serde::Serialize for GroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", &self.group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedPolicyArns", &self.managed_policy_arns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", &self.policies)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut group_name = None;
                let mut managed_policy_arns = None;
                let mut path = None;
                let mut policies = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GroupName" => {
                            group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ManagedPolicyArns" => {
                            managed_policy_arns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Path" => {
                            path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Policies" => {
                            policies = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(GroupProperties {
                    group_name: group_name,
                    managed_policy_arns: managed_policy_arns,
                    path: path,
                    policies: policies,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Group {
    type Properties = GroupProperties;
    const TYPE: &'static str = "AWS::IAM::Group";
    fn properties(&self) -> &GroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Group {}

impl From<GroupProperties> for Group {
    fn from(properties: GroupProperties) -> Group {
        Group { properties }
    }
}

/// The [`AWS::IAM::InstanceProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html) resource type.
#[derive(Debug)]
pub struct InstanceProfile {
    properties: InstanceProfileProperties
}

/// Properties for the `InstanceProfile` resource.
#[derive(Debug)]
pub struct InstanceProfileProperties {
    /// Property `InstanceProfileName`.
    pub instance_profile_name: Option<::Value<String>>,
    /// Property `Path`.
    pub path: Option<::Value<String>>,
    /// Property `Roles`.
    pub roles: ::ValueList<String>,
}

impl ::serde::Serialize for InstanceProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceProfileName", &self.instance_profile_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Roles", &self.roles)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_profile_name = None;
                let mut path = None;
                let mut roles = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceProfileName" => {
                            instance_profile_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Path" => {
                            path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Roles" => {
                            roles = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(InstanceProfileProperties {
                    instance_profile_name: instance_profile_name,
                    path: path,
                    roles: roles.ok_or(::serde::de::Error::missing_field("Roles"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InstanceProfile {
    type Properties = InstanceProfileProperties;
    const TYPE: &'static str = "AWS::IAM::InstanceProfile";
    fn properties(&self) -> &InstanceProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InstanceProfile {}

impl From<InstanceProfileProperties> for InstanceProfile {
    fn from(properties: InstanceProfileProperties) -> InstanceProfile {
        InstanceProfile { properties }
    }
}

/// The [`AWS::IAM::ManagedPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-managedpolicy.html) resource type.
#[derive(Debug)]
pub struct ManagedPolicy {
    properties: ManagedPolicyProperties
}

/// Properties for the `ManagedPolicy` resource.
#[derive(Debug)]
pub struct ManagedPolicyProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Groups`.
    pub groups: Option<::ValueList<String>>,
    /// Property `ManagedPolicyName`.
    pub managed_policy_name: Option<::Value<String>>,
    /// Property `Path`.
    pub path: Option<::Value<String>>,
    /// Property `PolicyDocument`.
    pub policy_document: ::Value<::json::Value>,
    /// Property `Roles`.
    pub roles: Option<::ValueList<String>>,
    /// Property `Users`.
    pub users: Option<::ValueList<String>>,
}

impl ::serde::Serialize for ManagedPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", &self.groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedPolicyName", &self.managed_policy_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Roles", &self.roles)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Users", &self.users)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ManagedPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ManagedPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ManagedPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ManagedPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut groups = None;
                let mut managed_policy_name = None;
                let mut path = None;
                let mut policy_document = None;
                let mut roles = None;
                let mut users = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Groups" => {
                            groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ManagedPolicyName" => {
                            managed_policy_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Path" => {
                            path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PolicyDocument" => {
                            policy_document = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Roles" => {
                            roles = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Users" => {
                            users = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ManagedPolicyProperties {
                    description: description,
                    groups: groups,
                    managed_policy_name: managed_policy_name,
                    path: path,
                    policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                    roles: roles,
                    users: users,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ManagedPolicy {
    type Properties = ManagedPolicyProperties;
    const TYPE: &'static str = "AWS::IAM::ManagedPolicy";
    fn properties(&self) -> &ManagedPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ManagedPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ManagedPolicy {}

impl From<ManagedPolicyProperties> for ManagedPolicy {
    fn from(properties: ManagedPolicyProperties) -> ManagedPolicy {
        ManagedPolicy { properties }
    }
}

/// The [`AWS::IAM::Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html) resource type.
#[derive(Debug)]
pub struct Policy {
    properties: PolicyProperties
}

/// Properties for the `Policy` resource.
#[derive(Debug)]
pub struct PolicyProperties {
    /// Property `Groups`.
    pub groups: Option<::ValueList<String>>,
    /// Property `PolicyDocument`.
    pub policy_document: ::Value<::json::Value>,
    /// Property `PolicyName`.
    pub policy_name: ::Value<String>,
    /// Property `Roles`.
    pub roles: Option<::ValueList<String>>,
    /// Property `Users`.
    pub users: Option<::ValueList<String>>,
}

impl ::serde::Serialize for PolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", &self.groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Roles", &self.roles)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Users", &self.users)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut groups = None;
                let mut policy_document = None;
                let mut policy_name = None;
                let mut roles = None;
                let mut users = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Groups" => {
                            groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PolicyDocument" => {
                            policy_document = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PolicyName" => {
                            policy_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Roles" => {
                            roles = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Users" => {
                            users = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(PolicyProperties {
                    groups: groups,
                    policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                    policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    roles: roles,
                    users: users,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Policy {
    type Properties = PolicyProperties;
    const TYPE: &'static str = "AWS::IAM::Policy";
    fn properties(&self) -> &PolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Policy {}

impl From<PolicyProperties> for Policy {
    fn from(properties: PolicyProperties) -> Policy {
        Policy { properties }
    }
}

/// The [`AWS::IAM::Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html) resource type.
#[derive(Debug)]
pub struct Role {
    properties: RoleProperties
}

/// Properties for the `Role` resource.
#[derive(Debug)]
pub struct RoleProperties {
    /// Property `AssumeRolePolicyDocument`.
    pub assume_role_policy_document: ::Value<::json::Value>,
    /// Property `ManagedPolicyArns`.
    pub managed_policy_arns: Option<::ValueList<String>>,
    /// Property `Path`.
    pub path: Option<::Value<String>>,
    /// Property `Policies`.
    pub policies: Option<::ValueList<self::role::Policy>>,
    /// Property `RoleName`.
    pub role_name: Option<::Value<String>>,
}

impl ::serde::Serialize for RoleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssumeRolePolicyDocument", &self.assume_role_policy_document)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedPolicyArns", &self.managed_policy_arns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", &self.policies)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleName", &self.role_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RoleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RoleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RoleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RoleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut assume_role_policy_document = None;
                let mut managed_policy_arns = None;
                let mut path = None;
                let mut policies = None;
                let mut role_name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssumeRolePolicyDocument" => {
                            assume_role_policy_document = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ManagedPolicyArns" => {
                            managed_policy_arns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Path" => {
                            path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Policies" => {
                            policies = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RoleName" => {
                            role_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(RoleProperties {
                    assume_role_policy_document: assume_role_policy_document.ok_or(::serde::de::Error::missing_field("AssumeRolePolicyDocument"))?,
                    managed_policy_arns: managed_policy_arns,
                    path: path,
                    policies: policies,
                    role_name: role_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Role {
    type Properties = RoleProperties;
    const TYPE: &'static str = "AWS::IAM::Role";
    fn properties(&self) -> &RoleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RoleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Role {}

impl From<RoleProperties> for Role {
    fn from(properties: RoleProperties) -> Role {
        Role { properties }
    }
}

/// The [`AWS::IAM::User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html) resource type.
#[derive(Debug)]
pub struct User {
    properties: UserProperties
}

/// Properties for the `User` resource.
#[derive(Debug)]
pub struct UserProperties {
    /// Property `Groups`.
    pub groups: Option<::ValueList<String>>,
    /// Property `LoginProfile`.
    pub login_profile: Option<::Value<self::user::LoginProfile>>,
    /// Property `ManagedPolicyArns`.
    pub managed_policy_arns: Option<::ValueList<String>>,
    /// Property `Path`.
    pub path: Option<::Value<String>>,
    /// Property `Policies`.
    pub policies: Option<::ValueList<self::user::Policy>>,
    /// Property `UserName`.
    pub user_name: Option<::Value<String>>,
}

impl ::serde::Serialize for UserProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", &self.groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoginProfile", &self.login_profile)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedPolicyArns", &self.managed_policy_arns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", &self.policies)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", &self.user_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut groups = None;
                let mut login_profile = None;
                let mut managed_policy_arns = None;
                let mut path = None;
                let mut policies = None;
                let mut user_name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Groups" => {
                            groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LoginProfile" => {
                            login_profile = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ManagedPolicyArns" => {
                            managed_policy_arns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Path" => {
                            path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Policies" => {
                            policies = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UserName" => {
                            user_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(UserProperties {
                    groups: groups,
                    login_profile: login_profile,
                    managed_policy_arns: managed_policy_arns,
                    path: path,
                    policies: policies,
                    user_name: user_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for User {
    type Properties = UserProperties;
    const TYPE: &'static str = "AWS::IAM::User";
    fn properties(&self) -> &UserProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for User {}

impl From<UserProperties> for User {
    fn from(properties: UserProperties) -> User {
        User { properties }
    }
}

/// The [`AWS::IAM::UserToGroupAddition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html) resource type.
#[derive(Debug)]
pub struct UserToGroupAddition {
    properties: UserToGroupAdditionProperties
}

/// Properties for the `UserToGroupAddition` resource.
#[derive(Debug)]
pub struct UserToGroupAdditionProperties {
    /// Property `GroupName`.
    pub group_name: ::Value<String>,
    /// Property `Users`.
    pub users: ::ValueList<String>,
}

impl ::serde::Serialize for UserToGroupAdditionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", &self.group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Users", &self.users)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserToGroupAdditionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserToGroupAdditionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserToGroupAdditionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserToGroupAdditionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut group_name = None;
                let mut users = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GroupName" => {
                            group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Users" => {
                            users = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(UserToGroupAdditionProperties {
                    group_name: group_name.ok_or(::serde::de::Error::missing_field("GroupName"))?,
                    users: users.ok_or(::serde::de::Error::missing_field("Users"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserToGroupAddition {
    type Properties = UserToGroupAdditionProperties;
    const TYPE: &'static str = "AWS::IAM::UserToGroupAddition";
    fn properties(&self) -> &UserToGroupAdditionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserToGroupAdditionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserToGroupAddition {}

impl From<UserToGroupAdditionProperties> for UserToGroupAddition {
    fn from(properties: UserToGroupAdditionProperties) -> UserToGroupAddition {
        UserToGroupAddition { properties }
    }
}

pub mod group {
    //! Property types for the `Group` resource.

    /// The [`AWS::IAM::Group.Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html) property type.
    #[derive(Debug)]
    pub struct Policy {
        /// Property `PolicyDocument`.
        pub policy_document: ::Value<::json::Value>,
        /// Property `PolicyName`.
        pub policy_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Policy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Policy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Policy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Policy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Policy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy_document = None;
                    let mut policy_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PolicyDocument" => {
                                policy_document = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PolicyName" => {
                                policy_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Policy {
                        policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                        policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod role {
    //! Property types for the `Role` resource.

    /// The [`AWS::IAM::Role.Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html) property type.
    #[derive(Debug)]
    pub struct Policy {
        /// Property `PolicyDocument`.
        pub policy_document: ::Value<::json::Value>,
        /// Property `PolicyName`.
        pub policy_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Policy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Policy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Policy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Policy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Policy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy_document = None;
                    let mut policy_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PolicyDocument" => {
                                policy_document = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PolicyName" => {
                                policy_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Policy {
                        policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                        policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user {
    //! Property types for the `User` resource.

    /// The [`AWS::IAM::User.LoginProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user-loginprofile.html) property type.
    #[derive(Debug)]
    pub struct LoginProfile {
        /// Property `Password`.
        pub password: ::Value<String>,
        /// Property `PasswordResetRequired`.
        pub password_reset_required: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for LoginProfile {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasswordResetRequired", &self.password_reset_required)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoginProfile {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoginProfile, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoginProfile;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoginProfile")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password = None;
                    let mut password_reset_required = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Password" => {
                                password = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PasswordResetRequired" => {
                                password_reset_required = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LoginProfile {
                        password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                        password_reset_required: password_reset_required,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IAM::User.Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html) property type.
    #[derive(Debug)]
    pub struct Policy {
        /// Property `PolicyDocument`.
        pub policy_document: ::Value<::json::Value>,
        /// Property `PolicyName`.
        pub policy_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Policy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Policy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Policy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Policy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Policy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy_document = None;
                    let mut policy_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PolicyDocument" => {
                                policy_document = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PolicyName" => {
                                policy_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Policy {
                        policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                        policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
