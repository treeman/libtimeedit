use serialize::{ Decodable, Decoder };
use std::fmt::{ Show, Formatter, Error };

#[deriving(Show, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    Course,
    Group,
}

impl Type {
    pub fn num_id(&self) -> uint {
        match *self {
            Type::Course => 219,
            Type::Group => 205,
        }
    }

    pub fn from_id(id: uint) -> Type {
        match id {
            219 => Type::Course,
            205 => Type::Group,
            _ => panic!("Unknown Type id {}", id),
        }
    }

    pub fn from_str(id: &str) -> Type {
        Type::from_id(from_str(id).unwrap())
    }
}

#[deriving(Clone, Eq, PartialEq)]
pub struct DataId {
    pub id: uint,
    pub typ: Type,
}

impl DataId {
    pub fn new(s: &str) -> DataId {
        let re = regex!(r"(\d+)\.(\d+)");
        let caps = match re.captures(s) {
            Some(x) => x,
            None => panic!("Cannot construct DataId from {}", s),
        };
        DataId {
            id: from_str(caps.at(1)).unwrap(),
            typ: Type::from_str(caps.at(2)),
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}.{}", self.id, self.typ)
    }
}

impl Show for DataId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}.{}", self.id, self.typ.num_id())
    }
}

impl<E, D:Decoder<E>> Decodable<D, E> for DataId {
    fn decode(d: &mut D) -> Result<DataId, E> {
        let s = try!(d.read_str());
        Ok(DataId::new(s[]))
    }
}

#[deriving(Clone, Eq, PartialEq)]
pub struct TypeInfo {
    pub code: String,
    pub name: String,
    pub id: DataId,
}

impl TypeInfo {
    pub fn new(code: &str, name: &str, id: DataId) -> TypeInfo {
        TypeInfo {
            code: code.to_string(),
            name: name.to_string(),
            id: id,
        }
    }
}

impl Show for TypeInfo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} \"{}\" id: {}", self.code, self.name, self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::{ DataId, TypeInfo, Type };

    #[test]
    fn test_typeinfo() {
        assert_eq!(TypeInfo::new("D3.a", "CIV ING UTB DATATEKNIK",
                                 DataId::new("1513.205")),
            TypeInfo {
                code: "D3.a".to_string(),
                name: "CIV ING UTB DATATEKNIK".to_string(),
                id: DataId::new("1513.205"),
            });

        assert_eq!(TypeInfo::new("TATA49", "Geometri och tillämpning",
                                 DataId::new("363741.219")),
            TypeInfo {
                code: "TATA49".to_string(),
                name: "Geometri och tillämpning".to_string(),
                id: DataId::new("363741.219"),
            });
    }

    #[test]
    fn test_dataid() {
        assert_eq!(DataId::new("1513.205"),
            DataId {
                id: 1513,
                typ: Type::from_id(205),
            }
        );
        assert_eq!(DataId::new("363741.219"),
            DataId {
                id: 363741,
                typ: Type::from_id(219),
            }
        );

        let x = format!("{}", DataId::new("1234.205"));
        assert_eq!(x[], "1234.205");
    }

    #[test]
    #[should_fail]
    fn test_dataid_fail() {
        let _x = DataId::new("12345");
    }
}

