use super::*;

impl EasyGraph {
    pub fn new(item: &ItemStruct) -> ParseResult<Self> {
        let name = item.ident.clone();

        let mut fields = item.fields.iter();
        let field = match fields.next() {
            Some(field) => field,
            None => return ParseResult::Bad(Error::new(item.span(), "Graph must have at least one field")),
        };

        let mut attrs = vec![];
        for i in &field.attrs {
            match GraphAttribute::new(i) {
                Some(s) => attrs.push(s),
                None => {}
            }
        }
        match attrs.as_slice() {
            [g] if g.is_easy_graph() => g.clone(),
            _ => return ParseResult::NotGood,
        };
        let field_name = field.ident.clone();
        let str_type = field.ty.to_token_stream().to_string();
        let field_type = match str_type.trim() {
            int @ ("i8" | "i16" | "i32" | "i64" | "i128" | "isize") => Ident::new(int, field.ty.span()),
            _ => return ParseResult::Bad(Error::new(field.ty.span(), "Easy graph type must be a signed integer")),
        };

        ParseResult::Ok(Self { graph_name: name.clone(), field_name, field_type })
    }
}
