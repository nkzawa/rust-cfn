//! Types for the `GameLift` service.

/// The [`AWS::GameLift::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-alias.html) resource type.
#[derive(Debug)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Debug)]
pub struct AliasProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `RoutingStrategy`.
    pub routing_strategy: ::Value<self::alias::RoutingStrategy>,
}

impl ::serde::Serialize for AliasProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoutingStrategy", &self.routing_strategy)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AliasProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AliasProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AliasProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AliasProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut name = None;
                let mut routing_strategy = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RoutingStrategy" => {
                            routing_strategy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(AliasProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    routing_strategy: routing_strategy.ok_or(::serde::de::Error::missing_field("RoutingStrategy"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Alias {
    type Properties = AliasProperties;
    const TYPE: &'static str = "AWS::GameLift::Alias";
    fn properties(&self) -> &AliasProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AliasProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Alias {}

impl From<AliasProperties> for Alias {
    fn from(properties: AliasProperties) -> Alias {
        Alias { properties }
    }
}

/// The [`AWS::GameLift::Build`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-build.html) resource type.
#[derive(Debug)]
pub struct Build {
    properties: BuildProperties
}

/// Properties for the `Build` resource.
#[derive(Debug)]
pub struct BuildProperties {
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `StorageLocation`.
    pub storage_location: Option<::Value<self::build::S3Location>>,
    /// Property `Version`.
    pub version: Option<::Value<String>>,
}

impl ::serde::Serialize for BuildProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageLocation", &self.storage_location)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BuildProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BuildProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BuildProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BuildProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name = None;
                let mut storage_location = None;
                let mut version = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StorageLocation" => {
                            storage_location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Version" => {
                            version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(BuildProperties {
                    name: name,
                    storage_location: storage_location,
                    version: version,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Build {
    type Properties = BuildProperties;
    const TYPE: &'static str = "AWS::GameLift::Build";
    fn properties(&self) -> &BuildProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BuildProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Build {}

impl From<BuildProperties> for Build {
    fn from(properties: BuildProperties) -> Build {
        Build { properties }
    }
}

/// The [`AWS::GameLift::Fleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html) resource type.
#[derive(Debug)]
pub struct Fleet {
    properties: FleetProperties
}

/// Properties for the `Fleet` resource.
#[derive(Debug)]
pub struct FleetProperties {
    /// Property `BuildId`.
    pub build_id: ::Value<String>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `DesiredEC2Instances`.
    pub desired_ec2_instances: ::Value<u32>,
    /// Property `EC2InboundPermissions`.
    pub ec2_inbound_permissions: Option<::ValueList<self::fleet::IpPermission>>,
    /// Property `EC2InstanceType`.
    pub ec2_instance_type: ::Value<String>,
    /// Property `LogPaths`.
    pub log_paths: Option<::ValueList<String>>,
    /// Property `MaxSize`.
    pub max_size: Option<::Value<u32>>,
    /// Property `MinSize`.
    pub min_size: Option<::Value<u32>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `ServerLaunchParameters`.
    pub server_launch_parameters: Option<::Value<String>>,
    /// Property `ServerLaunchPath`.
    pub server_launch_path: ::Value<String>,
}

impl ::serde::Serialize for FleetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildId", &self.build_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredEC2Instances", &self.desired_ec2_instances)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2InboundPermissions", &self.ec2_inbound_permissions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2InstanceType", &self.ec2_instance_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPaths", &self.log_paths)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSize", &self.max_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSize", &self.min_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerLaunchParameters", &self.server_launch_parameters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerLaunchPath", &self.server_launch_path)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FleetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FleetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FleetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FleetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut build_id = None;
                let mut description = None;
                let mut desired_ec2_instances = None;
                let mut ec2_inbound_permissions = None;
                let mut ec2_instance_type = None;
                let mut log_paths = None;
                let mut max_size = None;
                let mut min_size = None;
                let mut name = None;
                let mut server_launch_parameters = None;
                let mut server_launch_path = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BuildId" => {
                            build_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DesiredEC2Instances" => {
                            desired_ec2_instances = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EC2InboundPermissions" => {
                            ec2_inbound_permissions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EC2InstanceType" => {
                            ec2_instance_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LogPaths" => {
                            log_paths = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MaxSize" => {
                            max_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MinSize" => {
                            min_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ServerLaunchParameters" => {
                            server_launch_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ServerLaunchPath" => {
                            server_launch_path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(FleetProperties {
                    build_id: build_id.ok_or(::serde::de::Error::missing_field("BuildId"))?,
                    description: description,
                    desired_ec2_instances: desired_ec2_instances.ok_or(::serde::de::Error::missing_field("DesiredEC2Instances"))?,
                    ec2_inbound_permissions: ec2_inbound_permissions,
                    ec2_instance_type: ec2_instance_type.ok_or(::serde::de::Error::missing_field("EC2InstanceType"))?,
                    log_paths: log_paths,
                    max_size: max_size,
                    min_size: min_size,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    server_launch_parameters: server_launch_parameters,
                    server_launch_path: server_launch_path.ok_or(::serde::de::Error::missing_field("ServerLaunchPath"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Fleet {
    type Properties = FleetProperties;
    const TYPE: &'static str = "AWS::GameLift::Fleet";
    fn properties(&self) -> &FleetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FleetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Fleet {}

impl From<FleetProperties> for Fleet {
    fn from(properties: FleetProperties) -> Fleet {
        Fleet { properties }
    }
}

pub mod alias {
    //! Property types for the `Alias` resource.

    /// The [`AWS::GameLift::Alias.RoutingStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-alias-routingstrategy.html) property type.
    #[derive(Debug)]
    pub struct RoutingStrategy {
        /// Property `FleetId`.
        pub fleet_id: Option<::Value<String>>,
        /// Property `Message`.
        pub message: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for RoutingStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FleetId", &self.fleet_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", &self.message)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RoutingStrategy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RoutingStrategy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RoutingStrategy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RoutingStrategy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut fleet_id = None;
                    let mut message = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FleetId" => {
                                fleet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Message" => {
                                message = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(RoutingStrategy {
                        fleet_id: fleet_id,
                        message: message,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod build {
    //! Property types for the `Build` resource.

    /// The [`AWS::GameLift::Build.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-build-storagelocation.html) property type.
    #[derive(Debug)]
    pub struct S3Location {
        /// Property `Bucket`.
        pub bucket: ::Value<String>,
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `RoleArn`.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket = None;
                    let mut key = None;
                    let mut role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleArn" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod fleet {
    //! Property types for the `Fleet` resource.

    /// The [`AWS::GameLift::Fleet.IpPermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ec2inboundpermission.html) property type.
    #[derive(Debug)]
    pub struct IpPermission {
        /// Property `FromPort`.
        pub from_port: ::Value<u32>,
        /// Property `IpRange`.
        pub ip_range: ::Value<String>,
        /// Property `Protocol`.
        pub protocol: ::Value<String>,
        /// Property `ToPort`.
        pub to_port: ::Value<u32>,
    }

    impl ::codec::SerializeValue for IpPermission {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromPort", &self.from_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpRange", &self.ip_range)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ToPort", &self.to_port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IpPermission {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IpPermission, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IpPermission;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IpPermission")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut from_port = None;
                    let mut ip_range = None;
                    let mut protocol = None;
                    let mut to_port = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FromPort" => {
                                from_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IpRange" => {
                                ip_range = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Protocol" => {
                                protocol = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ToPort" => {
                                to_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(IpPermission {
                        from_port: from_port.ok_or(::serde::de::Error::missing_field("FromPort"))?,
                        ip_range: ip_range.ok_or(::serde::de::Error::missing_field("IpRange"))?,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                        to_port: to_port.ok_or(::serde::de::Error::missing_field("ToPort"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
