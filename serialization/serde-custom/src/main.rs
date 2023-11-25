use std::fmt::Formatter;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeStruct;

fn main() {
    let c = KubeConfig { port: 10, healthz_port: 11, max_pods: 12};
    let serialized = serde_json::to_string(&c).unwrap();
    println!("{:?}", serialized);
}

#[derive(Debug, PartialEq)]
struct KubeConfig {
    port: u8,
    healthz_port: u8,
    max_pods: u8,
}

impl Serialize for KubeConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("KubeConfig", 3)?;
        state.serialize_field("port", &self.port)?;
        state.serialize_field("healthz_port", &self.healthz_port)?;
        state.serialize_field("max_pods", &self.max_pods)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for KubeConfig {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        enum Filed { Port, HealthzPort, MaxPods }


        impl<'de> Deserialize<'de> for Filed {

            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Filed;

                    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                        formatter.write_str("`port` or `healthz_port` or `max_pods`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
                        const FIELDS: &'static [&'static str] = &["port", "healthz_port", "max_pods"];
                        match v {
                            "port" => Ok(Filed::Port),
                            "healthz_port" => Ok(Filed::HealthzPort),
                            "max_pods" => Ok(Filed::MaxPods),
                            _ => Err(de::Error::unknown_field(v, FIELDS))
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct KubeConfigVisitor;
        impl<'de> Visitor<'de> for KubeConfigVisitor {
            type Value = KubeConfig;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("struct KubeConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                let mut port = None;
                let mut hport = None;
                let mut max = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Filed::Port => {
                            if port.is_some() {
                                return Err(de::Error::duplicate_field("port"));
                            }
                            port = Some(map.next_value()?);
                        },
                        Filed::HealthzPort => {
                            if hport.is_some() {
                                return Err(de::Error::duplicate_field("healthz_port"));
                            }
                            hport = Some(map.next_value()?);
                        },
                        Filed::MaxPods => {
                            if max.is_some() {
                                return Err(de::Error::duplicate_field("max_pods"));
                            }
                            max = Some(map.next_value()?);
                        },
                    }
                }
                let port = port.ok_or_else(|| de::Error::missing_field("port"))?;
                let hport = hport.ok_or(de::Error::missing_field("healthz_port"))?;
                let max = max.ok_or(Error::missing_field("max_pods"))?;
                Ok(KubeConfig { port: port, healthz_port: hport, max_pods: max })
            }
        }
        const FIELDS: &'static [&'static str] = &["port", "healthz_port", "max_pods"];
        deserializer.deserialize_struct("KubeConfig", FIELDS, KubeConfigVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_de_tokens, Token};
    use crate::KubeConfig;

    #[test]
    fn test_ser_de() {
        let c = KubeConfig { port: 10, healthz_port: 11, max_pods: 12 };
        assert_de_tokens(&c, &[
            Token::Struct { name: "KubeConfig", len: 3 },
            Token::Str("port"),
            Token::U8(10),
            Token::Str("healthz_port"),
            Token::U8(11),
            Token::Str("max_pods"),
            Token::U8(12),
            Token::StructEnd,
        ]);
    }
}
