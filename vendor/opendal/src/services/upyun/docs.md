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
- `bucket`: Upyun bucket name
- `operator` Upyun operator
- `password` Upyun password

You can refer to [`UpyunBuilder`]'s docs for more information

## Example

### Via Builder

```rust
use anyhow::Result;
use opendal::services::Upyun;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Upyun::default();

    // set the storage bucket for OpenDAL
    builder.root("/");
    // set the bucket for OpenDAL
    builder.bucket("test");
    // set the operator for OpenDAL
    builder.operator("xxxxxxxxxx");
    // set the password name for OpenDAL
    builder.password("opendal");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```
