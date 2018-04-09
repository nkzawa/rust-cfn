//! Types for the `EFS` service.

/// The [`AWS::EFS::FileSystem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html) resource type.
#[derive(Debug)]
pub struct FileSystem {
    properties: FileSystemProperties
}

/// Properties for the `FileSystem` resource.
#[derive(Debug)]
pub struct FileSystemProperties {
    /// Property `Encrypted`.
    pub encrypted: Option<::Value<bool>>,
    /// Property `FileSystemTags`.
    pub file_system_tags: Option<::ValueList<self::file_system::ElasticFileSystemTag>>,
    /// Property `KmsKeyId`.
    pub kms_key_id: Option<::Value<String>>,
    /// Property `PerformanceMode`.
    pub performance_mode: Option<::Value<String>>,
}

impl ::serde::Serialize for FileSystemProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", &self.encrypted)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemTags", &self.file_system_tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", &self.kms_key_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceMode", &self.performance_mode)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FileSystemProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FileSystemProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FileSystemProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FileSystemProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut encrypted = None;
                let mut file_system_tags = None;
                let mut kms_key_id = None;
                let mut performance_mode = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Encrypted" => {
                            encrypted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "FileSystemTags" => {
                            file_system_tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KmsKeyId" => {
                            kms_key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PerformanceMode" => {
                            performance_mode = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(FileSystemProperties {
                    encrypted: encrypted,
                    file_system_tags: file_system_tags,
                    kms_key_id: kms_key_id,
                    performance_mode: performance_mode,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FileSystem {
    type Properties = FileSystemProperties;
    const TYPE: &'static str = "AWS::EFS::FileSystem";
    fn properties(&self) -> &FileSystemProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FileSystemProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FileSystem {}

impl From<FileSystemProperties> for FileSystem {
    fn from(properties: FileSystemProperties) -> FileSystem {
        FileSystem { properties }
    }
}

/// The [`AWS::EFS::MountTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-mounttarget.html) resource type.
#[derive(Debug)]
pub struct MountTarget {
    properties: MountTargetProperties
}

/// Properties for the `MountTarget` resource.
#[derive(Debug)]
pub struct MountTargetProperties {
    /// Property `FileSystemId`.
    pub file_system_id: ::Value<String>,
    /// Property `IpAddress`.
    pub ip_address: Option<::Value<String>>,
    /// Property `SecurityGroups`.
    pub security_groups: ::ValueList<String>,
    /// Property `SubnetId`.
    pub subnet_id: ::Value<String>,
}

impl ::serde::Serialize for MountTargetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemId", &self.file_system_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddress", &self.ip_address)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", &self.security_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MountTargetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MountTargetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MountTargetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MountTargetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut file_system_id = None;
                let mut ip_address = None;
                let mut security_groups = None;
                let mut subnet_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FileSystemId" => {
                            file_system_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IpAddress" => {
                            ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SecurityGroups" => {
                            security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetId" => {
                            subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(MountTargetProperties {
                    file_system_id: file_system_id.ok_or(::serde::de::Error::missing_field("FileSystemId"))?,
                    ip_address: ip_address,
                    security_groups: security_groups.ok_or(::serde::de::Error::missing_field("SecurityGroups"))?,
                    subnet_id: subnet_id.ok_or(::serde::de::Error::missing_field("SubnetId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MountTarget {
    type Properties = MountTargetProperties;
    const TYPE: &'static str = "AWS::EFS::MountTarget";
    fn properties(&self) -> &MountTargetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MountTargetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MountTarget {}

impl From<MountTargetProperties> for MountTarget {
    fn from(properties: MountTargetProperties) -> MountTarget {
        MountTarget { properties }
    }
}

pub mod file_system {
    //! Property types for the `FileSystem` resource.

    /// The [`AWS::EFS::FileSystem.ElasticFileSystemTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-filesystemtags.html) property type.
    #[derive(Debug)]
    pub struct ElasticFileSystemTag {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ElasticFileSystemTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElasticFileSystemTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticFileSystemTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticFileSystemTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticFileSystemTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticFileSystemTag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
