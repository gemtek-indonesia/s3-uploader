#[global_allocator]
static MEMORY_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

static ENVKEY_ACCESS_KEY_ID: &str = "S3_ACCESS_KEY_ID";
static ENVKEY_BUCKET_NAME: &str = "S3_BUCKET_NAME";
static ENVKEY_URL: &str = "S3_URL";
static ENVKEY_REGION: &str = "S3_REGION";
static ENVKEY_SECRET_ACCESS_KEY: &str = "S3_SECRET_ACCESS_KEY";
static ENVKEY_OBJECT_PATH: &str = "S3_OBJECT_PATH";
static ENVKEY_SOURCE_PATH: &str = "SOURCE_PATH";

fn get_env_var(key: &str) -> anyhow::Result<String> {
    std::env::var(key).map_err(|e| anyhow::anyhow!("Failed to get env var {}: {}", key, e))
}

struct Config {
    access_key_id: String,
    bucket_name: String,
    url: String,
    region: String,
    secret_access_key: String,
    object_path: String,
    source_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            access_key_id: get_env_var(ENVKEY_ACCESS_KEY_ID).unwrap(),
            bucket_name: get_env_var(ENVKEY_BUCKET_NAME).unwrap(),
            url: get_env_var(ENVKEY_URL).unwrap(),
            region: get_env_var(ENVKEY_REGION).unwrap(),
            secret_access_key: get_env_var(ENVKEY_SECRET_ACCESS_KEY).unwrap(),
            object_path: get_env_var(ENVKEY_OBJECT_PATH).unwrap(),
            source_path: get_env_var(ENVKEY_SOURCE_PATH).unwrap(),
        }
    }
}

async fn upload_file(config: Config) -> anyhow::Result<()> {
    let Config {
        access_key_id: s3_access_key_id,
        bucket_name: s3_bucket_name,
        url: s3_url,
        region: s3_region,
        secret_access_key: s3_secret_access_key,
        object_path,
        source_path,
    } = config;
    let s3_client = object_store::aws::AmazonS3Builder::default()
        .with_access_key_id(s3_access_key_id)
        .with_bucket_name(s3_bucket_name)
        .with_endpoint(s3_url)
        .with_region(s3_region)
        .with_secret_access_key(s3_secret_access_key)
        .with_virtual_hosted_style_request(false)
        .build()?;
    let source_object_bytes = tokio::fs::read(source_path).await?;
    let source_object_bytes = bytes::Bytes::from(source_object_bytes);
    let put_payload = object_store::PutPayload::from_bytes(source_object_bytes);
    let object_store_path = object_store::path::Path::from(object_path);
    object_store::ObjectStore::put(&s3_client, &object_store_path, put_payload).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::default();
    upload_file(config).await?;

    Ok(())
}
