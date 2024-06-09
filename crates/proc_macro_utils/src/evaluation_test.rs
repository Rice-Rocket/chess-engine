use syn::{parse_quote, ItemFn, LitStr};

pub fn expand(item: &mut ItemFn, args: LitStr) {
    let body = item.block.as_mut();
    *body = parse_quote!({
        crate::precomp::initialize();
        crate::move_gen::magics::initialize();
        let board = Board::load_position(Some(String::from(#args)), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board);
        eval.init::<White, Black>();

        #body
    });
}
