## Capabilities

This service can be used to:

- [x] stat
- [x] read
- [x] write
- [x] create_dir
- [x] delete
- [x] copy
- [x] rename
- [x] list
- [x] scan
- [ ] presign
- [ ] blocking

## Configuration

- `root`: Set the work directory for backend
- `endpoint`: Koofr endpoint
- `email` Koofr email
- `password` Koofr password

You can refer to [`KoofrBuilder`]'s docs for more information

## Example

### Via Builder

```rust
use anyhow::Result;
use opendal::services::Koofr;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Koofr::default();

    // set the storage bucket for OpenDAL
    builder.root("/");
    // set the bucket for OpenDAL
    builder.endpoint("https://api.koofr.net/");
    // set the email for OpenDAL
    builder.email("me@example.com");
    // set the password for OpenDAL
    builder.password("xxx xxx xxx xxx");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```
