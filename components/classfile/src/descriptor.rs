use std::fmt;
use std::fmt::Write;

#[derive(PartialEq, Clone)]
pub enum ComponentType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Reference(String),
    Short,
    Boolean,
}

#[derive(PartialEq, Clone)]
pub struct FieldType {
    dim: u8,
    ty: ComponentType,
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.dim {
            f.write_char('[')?;
        }

        match &self.ty {
            ComponentType::Byte => f.write_char('B')?,
            ComponentType::Char => f.write_char('C')?,
            ComponentType::Double => f.write_char('D')?,
            ComponentType::Float => f.write_char('F')?,
            ComponentType::Int => f.write_char('I')?,
            ComponentType::Long => f.write_char('J')?,
            ComponentType::Short => f.write_char('S')?,
            ComponentType::Boolean => f.write_char('Z')?,
            ComponentType::Reference(class_name) => {
                f.write_fmt(format_args!("L{};", class_name))?
            }
        }

        Ok(())
    }
}

impl fmt::Debug for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.ty {
            ComponentType::Byte => f.write_str("byte")?,
            ComponentType::Char => f.write_str("char")?,
            ComponentType::Double => f.write_str("double")?,
            ComponentType::Float => f.write_str("float")?,
            ComponentType::Int => f.write_str("int")?,
            ComponentType::Long => f.write_str("long")?,
            ComponentType::Short => f.write_str("short")?,
            ComponentType::Boolean => f.write_str("boolean")?,
            ComponentType::Reference(class_name) => f.write_str(&class_name.replace('/', "."))?,
        };

        for _ in 0..self.dim {
            f.write_str("[]")?;
        }

        Ok(())
    }
}

pub struct MethodDescriptor {
    pub params: Vec<FieldType>,
    pub rty: Option<FieldType>,
}

pub fn parse_field_descriptor_incomplete(x: &str) -> Option<(&str, FieldType)> {
    let dim = x.find(|c| c != '[')?;
    let x = &x[dim..];

    let (x, ty) = match x.chars().next()? {
        'B' => (&x[1..], ComponentType::Byte),
        'C' => (&x[1..], ComponentType::Char),
        'D' => (&x[1..], ComponentType::Double),
        'F' => (&x[1..], ComponentType::Float),
        'I' => (&x[1..], ComponentType::Int),
        'J' => (&x[1..], ComponentType::Long),
        'S' => (&x[1..], ComponentType::Short),
        'Z' => (&x[1..], ComponentType::Boolean),
        'L' => {
            let colon = x[1..].find(|c| c == ';')? + 1;
            (
                &x[colon + 1..],
                ComponentType::Reference(x[1..colon].to_string()),
            )
        }
        _ => return None,
    };

    Some((x, FieldType { dim: dim as u8, ty }))
}

pub fn parse_field_descriptor(x: &str) -> Option<FieldType> {
    match parse_field_descriptor_incomplete(x) {
        Some(("", field_type)) => Some(field_type),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
        let i = parse_field_descriptor_incomplete("I");
        assert_eq!(
            i,
            Some((
                "",
                FieldType {
                    dim: 0,
                    ty: ComponentType::Int
                }
            ))
        );
        let ty = i.unwrap().1;
        assert_eq!(&format!("{}", ty), "I");
        assert_eq!(&format!("{:?}", ty), "int");
    }

    #[test]
    fn reference() {
        let i = parse_field_descriptor_incomplete("Ljava/lang/Object;");
        assert_eq!(
            i,
            Some((
                "",
                FieldType {
                    dim: 0,
                    ty: ComponentType::Reference("java/lang/Object".into())
                }
            ))
        );
        let ty = i.unwrap().1;
        assert_eq!(&format!("{}", ty), "Ljava/lang/Object;");
        assert_eq!(&format!("{:?}", ty), "java.lang.Object");
    }

    #[test]
    fn array() {
        let i = parse_field_descriptor_incomplete("[[[D");
        assert_eq!(
            i,
            Some((
                "",
                FieldType {
                    dim: 3,
                    ty: ComponentType::Double
                }
            ))
        );

        let ty = i.unwrap().1;
        assert_eq!(&format!("{}", ty), "[[[D");
        assert_eq!(&format!("{:?}", ty), "double[][][]");
    }

    #[test]
    fn reference_array() {
        let i = parse_field_descriptor_incomplete("[Ljava/lang/Object;");
        assert_eq!(
            i,
            Some((
                "",
                FieldType {
                    dim: 1,
                    ty: ComponentType::Reference("java/lang/Object".into())
                }
            ))
        );

        let ty = i.unwrap().1;
        assert_eq!(&format!("{}", ty), "[Ljava/lang/Object;");
        assert_eq!(&format!("{:?}", ty), "java.lang.Object[]");
    }

    #[test]
    fn missing_ref_end() {
        let i = parse_field_descriptor_incomplete("[Ljava/lang/Object");
        assert_eq!(i, None);
    }

    #[test]
    fn trailing() {
        let i = parse_field_descriptor_incomplete("[Ljava/lang/Object;[[Ljava/lang/Object;");
        assert_eq!(
            i,
            Some((
                "[[Ljava/lang/Object;",
                FieldType {
                    dim: 1,
                    ty: ComponentType::Reference("java/lang/Object".into())
                }
            ))
        );
    }

    #[test]
    fn trailing_on_complete() {
        let i = parse_field_descriptor("[Ljava/lang/Object;[[Ljava/lang/Object;");
        assert_eq!(i, None);
    }
}
