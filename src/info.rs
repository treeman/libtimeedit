use std::fmt::{Show, Formatter, FormatError};
use time;
use time::Tm;

#[deriving(Show, Clone, Eq, PartialEq)]
pub enum Type {
    Course,
    Group,
}

impl Type {
    pub fn num_id(&self) -> uint {
        match *self {
            Course => 219,
            Group => 205,
        }
    }

    pub fn from_id(id: uint) -> Type {
        match id {
            219 => Course,
            205 => Group,
            _ => panic!("Unknown Type id {}", id),
        }
    }
}

#[deriving(Show, Clone, Eq, PartialEq)]
pub struct TypeInfo {
    pub id: String,
    pub name: String,
    pub data_id: String,
    pub typ: Type,
}

impl TypeInfo {
    pub fn new(id: &str, name: &str, data_id: &str) -> TypeInfo {
        let p = data_id.find('.').unwrap();
        let type_id = from_str(data_id.slice_from_or_fail(&(p + 1))).unwrap();

        TypeInfo {
            id: id.to_string(),
            name: name.to_string(),
            data_id: data_id.to_string(),
            typ: Type::from_id(type_id),
        }
    }
}

pub struct Entry {
    pub start: Tm,
    pub end: Tm,
    pub name: String,
    pub loc: String,
}

impl Show for Entry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        let date_format = "%F %R";
        write!(f, "{} - {} {}",
            time::strftime(date_format, &self.start).unwrap(),
            time::strftime(date_format, &self.end).unwrap(),
            self.name
        )
    }
}

#[test]
fn test_typeinfo() {
    assert_eq!(TypeInfo::new("D3.a", "CIV ING UTB DATATEKNIK", "1513.205"),
        TypeInfo {
            id: "D3.a".to_string(),
            name: "CIV ING UTB DATATEKNIK".to_string(),
            data_id: "1513.205".to_string(),
            typ: Group,
        });

    assert_eq!(TypeInfo::new("TATA49", "Geometri och tillämpning", "363741.219"),
        TypeInfo {
            id: "TATA49".to_string(),
            name: "Geometri och tillämpning".to_string(),
            data_id: "363741.219".to_string(),
            typ: Course,
        });
}

