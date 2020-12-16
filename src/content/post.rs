imports!();

use crate::client::PostQueryBuilder;

new_type!(
    Attachment
    AttachmentId
    Child
    Content
    ContentId
    Data
);

from!(
    @PostQueryBuilder
        -> Content = "content"
    @Content
        => ContentId
    @ContentId
        -> Child = "child"
    @Child
        -> Attachment = "attachment"
    @Attachment
        => AttachmentId
    @AttachmentId
        -> Data = "data"
);

impl_macro!(
    @Content
        |
        |=> content_id -> ContentId = content_id_str
    @ContentId
        |=> child -> Child
        |
    @Child
        |
        |~> attachment -> Attachment = form
    @Attachment
        |
        |=> attachment_id -> AttachmentId = attachment_id_str
    @AttachmentId
        |=> data -> Data
        |

);

exec!(Content);
exec!(Attachment);
exec!(Data);
