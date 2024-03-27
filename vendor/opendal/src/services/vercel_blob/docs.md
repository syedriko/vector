## Capabilities

This service can be used to:

- [x] stat
- [x] read
- [x] write
- [x] create_dir
- [x] delete
- [x] copy
- [ ] rename
- [x] list
- [x] scan
- [ ] presign
- [ ] blocking

## Configuration

- `root`: Set the work directory for backend
- `token`: VercelBlob token, environment var `BLOB_READ_WRITE_TOKEN`

You can refer to [`VercelBlobBuilder`]'s docs for more information

## Example

### Via Builder

```rust
use anyhow::Result;
use opendal::services::VercelBlob;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = VercelBlob::default();

    // set the storage bucket for OpenDAL
    builder.root("/");
    // set the token for OpenDAL
    builder.token("you_token");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```
