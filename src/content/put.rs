imports!();

use crate::client::PutQueryBuilder;

new_type!(
    Content
    Id
);

from!(
    @PutQueryBuilder
        -> Content = "content"
    @Content
        => Id
);

impl_macro!(
    @Content
        |
        |=> content_id -> Id = content_id_str
);

exec!(Content);
exec!(Id);
