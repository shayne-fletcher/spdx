%start LicenseExpr

%%

// [AST license expressions specification](https://ast.github.io/ast-spec/v2.3/AST-license-expressions/)

LicenseExpr -> Result<ast::LicenseExpr>:
   CompoundExpr {
     let compound_expr = $1?;
     if let ast::CompoundExpr::SimpleExpr(simple_expr) = compound_expr {
         Ok(ast::LicenseExpr::SimpleExpr(simple_expr))
     } else {
       Ok(ast::LicenseExpr::CompoundExpr(compound_expr))
     }
   }
  ;

CompoundExpr -> Result<ast::CompoundExpr>:
    CompoundExpr 'OR' AExpr {
        let lexpr = Box::new($1?);
        let rexpr = Box::new($3?);
        Ok(ast::CompoundExpr::OrExpr(ast::OrExpr{lexpr, rexpr}))
    }
  | CompoundExpr '/' AExpr {
        // This syntax is unofficial but it occurs on crates.io. e.g.
        // 'unicode-noramlization' reports license 'MIT/Apache-2.0'
        let lexpr = Box::new($1?);
        let rexpr = Box::new($3?);
        Ok(ast::CompoundExpr::OrExpr(ast::OrExpr{lexpr, rexpr}))
    }
  | AExpr { $1 }
  ;

AExpr -> Result<ast::CompoundExpr>:
    AExpr 'AND' BExpr {
        let lexpr = Box::new($1?);
        let rexpr = Box::new($3?);
        Ok(ast::CompoundExpr::AndExpr(ast::AndExpr{lexpr, rexpr}))
    }
  | BExpr { $1 }
  ;

BExpr -> Result<ast::CompoundExpr>:
    SimpleExpr 'WITH' Identifier {
        let expr = Box::new($1?);
        let id = crate::LicenseExceptionId($3?);
        let license_exception_id = ast::LicenseExceptionId{id};
        Ok(ast::CompoundExpr::SimpleExprWithException(
             ast::SimpleExprWithException{expr, license_exception_id}))
    }
  | '(' CompoundExpr ')' {
      let expr = Box::new($2?);
      Ok(ast::CompoundExpr::ParenExpr(ast::ParenExpr{expr}))
    }
  | SimpleExpr {
        Ok(ast::CompoundExpr::SimpleExpr($1?))
    }
  ;

SimpleExpr -> Result<ast::SimpleExpr>:
    Identifier ':' Identifier { // license-ref
      let prefix = $1?;
      let suffix = $3?;
      let document_ref = Some(prefix.strip_prefix(DOCUMENT_REF).ok_or(Box::<dyn std::error::Error>::from(format!("'DocumentRef-' expected got '{prefix}'")))?.to_owned());
      let license_ref = suffix.strip_prefix(LICENSE_REF).ok_or(Box::<dyn std::error::Error>::from(format!("'License-Ref-' expected got '{suffix}'")))?.to_owned();
      if let Some(document_ref) = &document_ref {
          if document_ref.is_empty() {
              return Err("'DocumentRef-' suffix is empty".into());
            }
        }
      if license_ref.is_empty() {
          return Err(Box::<dyn std::error::Error>::from("'LicenseRef-' suffix is empty"));
        }
      Ok(ast::SimpleExpr::LicenseRef(ast::LicenseRef{document_ref, license_ref}))
    }
  | Identifier '+' { //license-id+
      let id = crate::LicenseId($1?);
      Ok(ast::SimpleExpr::LicenseId(ast::LicenseId{id, plus: true}))
    }
  | Identifier { //license-id or license-ref
        let license_str = $1?;
        if license_str.starts_with(LICENSE_REF) {
            let license_ref = license_str.strip_prefix(LICENSE_REF).unwrap().to_owned();
            if license_ref.is_empty() {
                return Err("'LicenseRef-' suffix is empty".into());
              }
            Ok(ast::SimpleExpr::LicenseRef(ast::LicenseRef {document_ref:None, license_ref}))
          }
        else {
            Ok(ast::SimpleExpr::LicenseId(ast::LicenseId {id: crate::LicenseId(license_str), plus: false, }))
        }
     }
  ;

Identifier -> Result<String>:
  'IDENTIFIER' {
      let v = $1?;
      Ok($lexer.span_str(v.span()).to_owned())
    }
  ;

Unmatched -> ():
  "UNMATCHED" { }
  ;

%%

pub static LICENSE_REF: &str = "LicenseRef-";
pub static DOCUMENT_REF: &str = "DocumentRef-";

pub type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;

pub mod ast {

  #[derive(Debug)]
  pub enum LicenseExpr {
    SimpleExpr(SimpleExpr),
    CompoundExpr(CompoundExpr),
  }

  #[derive(Debug)]
  pub enum CompoundExpr {
    SimpleExpr(SimpleExpr),
    SimpleExprWithException(SimpleExprWithException),
    OrExpr(OrExpr),
    AndExpr(AndExpr),
    ParenExpr(ParenExpr),
  }

  #[derive(Debug)]
  pub struct ParenExpr {
    pub expr: Box<CompoundExpr>,
  }

  #[derive(Debug)]
  pub struct OrExpr {
      pub lexpr: Box<CompoundExpr>,
      pub rexpr: Box<CompoundExpr>,
  }

  #[derive(Debug)]
  pub struct AndExpr {
      pub lexpr: Box<CompoundExpr>,
      pub rexpr: Box<CompoundExpr>,
  }

  #[derive(Debug)]
  pub struct SimpleExprWithException {
      pub expr: Box<SimpleExpr>,
      pub license_exception_id: LicenseExceptionId,
  }

  #[derive(Debug)]
  pub enum SimpleExpr {
      LicenseId(LicenseId),
      LicenseRef(LicenseRef),
  }

  #[derive(Debug)]
  pub struct LicenseId {
      pub id: crate::LicenseId,
      pub plus: bool,
  }

  #[derive(Debug)]
  pub struct LicenseExceptionId {
      pub id: crate::LicenseExceptionId,
  }

  #[derive(Debug)]
  pub struct LicenseRef {
      pub document_ref: Option<String>,
      pub license_ref: String,
  }

}
