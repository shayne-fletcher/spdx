%start LicenseExpr
%%

// [AST license expressions specification](https://ast.github.io/ast-spec/v2.3/AST-license-expressions/)

LicenseExpr -> Result<ast::LicenseExpr, ()>:
   CompoundExpr {
     let compound_expr = $1?;
     if let ast::CompoundExpr::SimpleExpr(simple_expr) = compound_expr {
         Ok(ast::LicenseExpr::SimpleExpr(simple_expr))
     } else {
       Ok(ast::LicenseExpr::CompoundExpr(compound_expr))
     }
   }
  ;

CompoundExpr -> Result<ast::CompoundExpr, ()>:
    CompoundExpr 'OR' Term {
        let lexpr = Box::new($1?);
        let rexpr = Box::new($3?);
        Ok(ast::CompoundExpr::OrExpr(ast::OrExpr{lexpr, rexpr}))
    }
  | Term { $1 }
  ;

Term -> Result<ast::CompoundExpr, ()>:
    Term 'AND' Factor {
        let lexpr = Box::new($1?);
        let rexpr = Box::new($3?);
        Ok(ast::CompoundExpr::AndExpr(ast::AndExpr{lexpr, rexpr}))
    }
    | Factor { $1 }
  ;

Factor -> Result<ast::CompoundExpr, ()>:
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

SimpleExpr -> Result<ast::SimpleExpr, ()>:
    Identifier ':' Identifier { // license-ref
      let document_ref = Some($1?.strip_prefix(DOCUMENT_REF).ok_or(())?.to_owned());
      let license_ref = $3?.strip_prefix(LICENSE_REF).ok_or(())?.to_owned();
      Ok(ast::SimpleExpr::LicenseRef(ast::LicenseRef{document_ref, license_ref}))
    }
  | Identifier '+' { //license-id+
      let id = crate::LicenseId($1?);
      Ok(ast::SimpleExpr::LicenseId(ast::LicenseId{id, plus: true}))
    }
  | Identifier { //license-id or license-ref
        let license_str = $1?;
        if license_str.starts_with(LICENSE_REF) {
            let license_ref = license_str.strip_prefix(LICENSE_REF).ok_or(())?.to_owned();
            Ok(ast::SimpleExpr::LicenseRef(ast::LicenseRef {document_ref:None, license_ref}))
          }
        else {
            Ok(ast::SimpleExpr::LicenseId(ast::LicenseId {id: crate::LicenseId(license_str), plus: false, }))
        }
     }
  ;

Identifier -> Result<String, ()>:
  'IDENTIFIER' {
      let v = $1.map_err(|_| ())?;
      Ok($lexer.span_str(v.span()).to_owned())
    }
  ;

%%

static LICENSE_REF: &str = "LicenseRef-";
static DOCUMENT_REF: &str = "DocumentRef-";

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
