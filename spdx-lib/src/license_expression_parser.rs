use lrpar::lrpar_mod;

lrpar_mod!("spdx.y");

pub use ast::*;
pub use spdx_y::*;

impl std::str::FromStr for LicenseExpr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lexerdef = super::license_expression_lexer::lexerdef();
        let lexer = lexerdef.lexer(s);
        let (res, _) = parse(&lexer);
        match res {
            Some(Ok(r)) => Ok(r),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for LicenseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.id.fmt(f)?;
        if self.plus {
            write!(f, "+")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for LicenseExceptionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.id.fmt(f)
    }
}

impl std::fmt::Display for LicenseRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.document_ref {
            Some(ref document_ref) => {
                write!(
                    f,
                    "DocumentRef-{}:LicenseRef-{}",
                    document_ref, self.license_ref
                )
            }
            None => write!(f, "LicenseRef-{}", self.license_ref),
        }
    }
}

impl std::fmt::Display for SimpleExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SimpleExpr::LicenseId(l) => l.fmt(f),
            SimpleExpr::LicenseRef(l) => l.fmt(f),
        }
    }
}

impl std::fmt::Display for ParenExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({})", self.expr)
    }
}

impl std::fmt::Display for SimpleExprWithException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} WITH {}", self.expr, self.license_exception_id)
    }
}

impl std::fmt::Display for AndExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} AND {}", self.lexpr, self.rexpr)
    }
}

impl std::fmt::Display for OrExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} OR {}", self.lexpr, self.rexpr)
    }
}

impl std::fmt::Display for LicenseExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LicenseExpr::SimpleExpr(e) => e.fmt(f),
            LicenseExpr::CompoundExpr(e) => e.fmt(f),
        }
    }
}

impl std::fmt::Display for CompoundExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CompoundExpr::SimpleExpr(e) => e.fmt(f),
            CompoundExpr::SimpleExprWithException(e) => e.fmt(f),
            CompoundExpr::OrExpr(e) => e.fmt(f),
            CompoundExpr::AndExpr(e) => e.fmt(f),
            CompoundExpr::ParenExpr(e) => e.fmt(f),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ast::*;

    #[test]
    fn test_roundtrip_simple_0() {
        let s = "LicenseRef-23";
        assert_eq!(s.parse::<LicenseExpr>().unwrap().to_string(), s)
    }

    #[test]
    fn test_roundtrip_simple_1() {
        let s = "LicenseRef-MIT-Style-1";
        assert_eq!(s.parse::<LicenseExpr>().unwrap().to_string(), s)
    }

    #[test]
    fn test_roundtrip_simple_2() {
        let s = "DocumentRef-spdx-tool-1.2:LicenseRef-MIT-Style-2";
        assert_eq!(s.parse::<LicenseExpr>().unwrap().to_string(), s)
    }

    #[test]
    fn test_roundtrip_composite_or_0() {
        let s = "LGPL-2.1-only OR MIT";
        assert_eq!(s.parse::<LicenseExpr>().unwrap().to_string(), s)
    }

    #[test]
    fn test_roundtrip_composite_or_1() {
        let s = "LGPL-2.1-only OR MIT OR BSD-3-Clause";
        assert_eq!(s.parse::<LicenseExpr>().unwrap().to_string(), s)
    }

    #[test]
    fn test_roundtrip_composite_and_0() {
        let s = "LGPL-2.1-only AND MIT";
        assert_eq!(s.parse::<LicenseExpr>().unwrap().to_string(), s)
    }

    #[test]
    fn test_roundtrip_composite_and_1() {
        let s = "LGPL-2.1-only AND MIT AND BSD-2-Clause";
        assert_eq!(s.parse::<LicenseExpr>().unwrap().to_string(), s)
    }

    #[test]
    fn test_roundtrip_composite_exception_with_0() {
        let s = "GPL-2.0-or-later WITH Bison-exception-2.2";
        assert_eq!(s.parse::<LicenseExpr>().unwrap().to_string(), s)
    }

    #[test]
    fn test_roundtrip_composite_parens_0() {
        let s = "MIT AND (LGPL-2.1-or-later OR BSD-3-Clause)";
        assert_eq!(s.parse::<LicenseExpr>().unwrap().to_string(), s)
    }
}
