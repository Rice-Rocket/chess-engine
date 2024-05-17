use syn::{parse_quote, ItemFn, LitStr};

pub fn expand(item: &mut ItemFn, args: LitStr) {
    let body = item.block.as_mut();
    *body = parse_quote!({
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from(#args)), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics);
        eval.init::<White, Black>();

        #body
    });
}
