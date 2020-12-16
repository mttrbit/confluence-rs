imports!();

use crate::client::GetQueryBuilder;

new_type!(
    Attachment
    Content
    Id
    Child
    Expand
    Title
    SpaceKey
    Filename
);

from!(
    @Attachment
        ?> Filename = "filename"
    @GetQueryBuilder
        -> Content = "content"
    @Content
        => Id
    @SpaceKey
        ?> Title = "title"
    @Content
        ?> SpaceKey = "spaceKey"
    @Id
        -> Child = "child"
    @Child
        ?> Expand = "expand"
    @Child
        -> Attachment = "attachment"
    @Title
        ?> Expand = "expand"
);

impl_macro!(
    @Attachment
        |
        |?> filename -> Filename = filename_str
    @Content
        |
        |=> content_id -> Id = content_id_str
        |?> space_key -> SpaceKey = space_key_str
    @Id
        |=> child -> Child
        |
    @Child
        |=> attachment -> Attachment
        |
    @Child
        |
        |?> expand -> Expand = expand
    @SpaceKey
        |
        |?> title -> Title = title_str
    @Title
        |
        |?> expand -> Expand = expand
);

exec!(Content);
exec!(Expand);
exec!(Attachment);
exec!(Filename);
