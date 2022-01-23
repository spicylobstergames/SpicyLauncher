#[derive(Debug)]
pub struct Asset {
    pub name: String,
    pub download_url: String,
    pub size: i64,
}

#[derive(Debug)]
pub struct Release {
    pub name: String,
    pub version: String,
    pub assets: Vec<Asset>,
}
