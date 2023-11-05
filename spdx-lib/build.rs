use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;

fn main() {
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                // .recoverer(lrpar::RecoveryKind::None)
                .grammar_in_src_dir("spdx.y")
                .unwrap()
        })
        .lexer_in_src_dir("spdx.l")
        .unwrap()
        .build()
        .unwrap();
}
