use snarkvm::prelude::{Block, Deserialize};
use snarkvm::prelude::MainnetV0 as CurrentNetwork;
use anyhow::{Context, Result};

use std::time::Duration;
use snarkvm::ledger::puzzle::Solution;

#[derive(Clone)]
pub struct AleoRpcClient {
    base_url: String,
    inner: reqwest::Client,
}

impl AleoRpcClient {
    pub fn new(base_url: &str) -> Self {
        // ex: https://vm.aleo.org/api/testnet3
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            inner: reqwest::Client::builder().timeout(Duration::from_secs(5)).build().unwrap(),
        }
    }

    pub async fn get_resource<R: for<'a> Deserialize<'a>>(&self, url: &str) -> Result<R> {
        let resp = self.inner.get(url).send().await?;
        let status = resp.status();
        let data = resp.text().await.context("get resource to text")?;
        let resource = match status.is_success() {
            true => serde_json::from_str::<R>(&data).with_context(move || format!("serialize data to resource: {}", data))?,
            false => return Err(anyhow::anyhow!("request {} failed, status: {}, body: {}", &url, status, data)),
        };
        Ok(resource)
    }
}

impl AleoRpcClient {
    pub async fn get_block(&self, block_height: u32) -> Result<Block<CurrentNetwork>> {
        let url = format!("{}/block/{}", self.base_url, block_height);
        let block = self.get_resource(&url).await?;
        Ok(block)
    }

    pub async fn get_blocks(&self, start_height: u32, end_height: u32) -> Result<Vec<Block<CurrentNetwork>>> {
        let url = format!("{}/blocks?start={start_height}&end={end_height}", self.base_url);
        let blocks = self.get_resource(&url).await?;
        Ok(blocks)
    }


    pub async fn broadcast_solution(&self, solution: Solution<CurrentNetwork>) -> Result<()> {
        let url = format!("{}/solution/broadcast", self.base_url);
        let resp = self.inner.post(&url).json(&solution).send().await?;
        let status = resp.status();
        let data = resp.text().await.context("get resource to text")?;
        match status.is_success() {
            true => Ok(()),
            false => Err(anyhow::anyhow!("request {} failed, status: {}, body: {}", &url, status, data)),
        }
    }

    pub async fn get_latest_height(&self) -> Result<u32> {
        let url = format!("{}/block/height/latest", self.base_url);
        let height = self.get_resource(&url).await?;
        Ok(height)
    }

    pub async fn get_latest_block(&self) -> Result<Block<CurrentNetwork>> {
        let url = format!("{}/block/latest", self.base_url);
        let block = self.get_resource(&url).await?;
        Ok(block)
    }
}
