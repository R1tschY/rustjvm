use std::fmt;
use std::fmt::Write;

pub trait GenJavaCode {
    fn gen_java_code<W: Write>(&self, writer: &mut W) -> fmt::Result;
}

impl<T: GenJavaCode> GenJavaCode for Option<T> {
    fn gen_java_code<W: Write>(&self, writer: &mut W) -> fmt::Result {
        if let Some(inner) = &self {
            inner.gen_java_code(writer)?;
            writer.write_char(' ')
        } else {
            Ok(())
        }
    }
}

pub struct JavaFile {
    pub package: String,
    pub imports: Vec<JavaImport>,
    pub classes: Vec<JavaClass>,
}

impl GenJavaCode for JavaFile {
    fn gen_java_code<W: Write>(&self, writer: &mut W) -> fmt::Result {
        writer.write_fmt(format_args!("package {};\n\n", self.package))?;
        // for import in &self.imports {
        //     import.gen_code(writer);
        // }

        Ok(())
    }
}

pub enum JavaImport {
    Static(String),
    Class(String),
}

impl GenJavaCode for JavaImport {
    fn gen_java_code<W: Write>(&self, writer: &mut W) -> fmt::Result {
        match self {
            JavaImport::Static(s) => writer.write_fmt(format_args!("import static {};\n", s)),
            JavaImport::Class(s) => writer.write_fmt(format_args!("import {};\n", s)),
        }
    }
}

pub enum JavaVisibility {
    Public,
    Protected,
    Private,
}

impl GenJavaCode for JavaVisibility {
    fn gen_java_code<W: Write>(&self, writer: &mut W) -> fmt::Result {
        match self {
            JavaVisibility::Private => writer.write_str("private "),
            JavaVisibility::Protected => writer.write_str("protected "),
            JavaVisibility::Public => writer.write_str("public "),
        }
    }
}

pub struct JavaClass {
    pub name: String,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub fields: Vec<JavaField>,
    pub methods: Vec<JavaMethod>,
    pub static_init: Option<JavaBlock>,
}

impl GenJavaCode for JavaClass {
    fn gen_java_code<W: Write>(&self, writer: &mut W) -> fmt::Result {
        writer.write_fmt(format_args!("class {} ", self.name))?;
        if let Some(extends) = &self.extends {
            writer.write_fmt(format_args!("extends {} ", extends))?;
        }
        if !self.implements.is_empty() {
            writer.write_str("implements ")?;
            // TODO: join(writer, ", ", &self.implements)?;
        }
        writer.write_str("{\n")?;

        if let Some(static_init) = &self.static_init {
            writer.write_str("static ")?;
            // TODO: static_init.gen_java_code(writer)?;
        }

        for field in &self.fields {
            field.gen_java_code(writer);
        }

        for method in &self.methods {
            method.gen_java_code(writer);
        }

        writer.write_str("}\n")
    }
}

pub struct JavaField {
    vis: Option<JavaVisibility>,
    static_: bool,
    ty: String,
    ident: String,
    init: Option<JavaExpr>,
}

impl GenJavaCode for JavaField {
    fn gen_java_code<W: Write>(&self, writer: &mut W) -> fmt::Result {
        self.vis.gen_java_code(writer)?;
        if self.static_ {
            writer.write_str("static ")?;
        }
        writer.write_fmt(format_args!("{} {}", self.ty, self.ident))?;
        if let Some(init) = &self.init {
            writer.write_str(" = ")?;
            // TODO: init.gen_java_code(writer)?;
        }
        writer.write_str(";\n")
    }
}

pub struct JavaArgument {
    ty: String,
    ident: String,
}

impl GenJavaCode for JavaArgument {
    fn gen_java_code<W: Write>(&self, writer: &mut W) -> fmt::Result {
        writer.write_fmt(format_args!("{} {}", self.ty, self.ident))
    }
}

pub struct JavaMethod {
    vis: Option<JavaVisibility>,
    static_: bool,
    final_: bool,
    rty: String,
    ident: String,
    args: Vec<JavaArgument>,
    body: JavaBlock,
}

fn join<T: GenJavaCode, W: Write>(writer: &mut W, sep: &str, elems: &[T]) -> fmt::Result {
    let mut first = true;
    for elem in &elems[1..] {
        if first {
            writer.write_str(sep)?;
        } else {
            first = false;
        }
        elem.gen_java_code(writer)?;
    }
    Ok(())
}

impl GenJavaCode for JavaMethod {
    fn gen_java_code<W: Write>(&self, writer: &mut W) -> fmt::Result {
        self.vis.gen_java_code(writer)?;
        if self.final_ {
            writer.write_str("final ")?;
        }
        if self.static_ {
            writer.write_str("static ")?;
        }
        writer.write_fmt(format_args!("{} {}(", self.rty, self.ident))?;
        join(writer, ", ", &self.args)?;
        writer.write_str(") ")
        // TODO: self.body.gen_java_code(writer)
    }
}

pub struct JavaBlock {
    items: Vec<JavaStatement>,
}

pub enum JavaStatement {
    Expr(JavaExpr),
    Assign {
        ty: String,
        ident: String,
        expr: JavaExpr,
    },
    For {},
    ForIn {},
    While {
        cond: JavaExpr,
        block: Box<JavaBlock>,
    },
    Return(JavaExpr),
}

pub enum JavaExpr {
    Call {
        callee: Box<JavaExpr>,
        parameters: Vec<JavaExpr>,
    },
    Cast {
        ty: String,
        expr: Box<JavaExpr>,
    },
    BinOp {
        left: Box<JavaExpr>,
        right: Box<JavaExpr>,
    },
}
