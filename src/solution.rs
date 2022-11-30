use crate::statement::*;
use async_trait::async_trait;
use tokio::sync::{mpsc, watch};

const MAX_DOWNLOAD_ATTEMPTS: usize = 3;

#[async_trait]
pub trait Solution {
    async fn solve(repositories: Vec<ServerName>) -> Option<Binary>;
    async fn download_task(
        repository: ServerName,
        cancel: watch::Receiver<bool>,
        tx_binary: mpsc::Sender<Option<Binary>>,
    );
    async fn spawn_all_downloads(
        repositories: Vec<ServerName>,
        rx_cancel: watch::Receiver<bool>,
        tx_binary: mpsc::Sender<Option<Binary>>,
    );
    async fn cancel_tasks(tx_cancel: &watch::Sender<bool>);
    async fn receive_downloaded_data(
        rx: &mut mpsc::Receiver<Option<Binary>>,
        tx_cancel: watch::Sender<bool>,
        total_downloads: usize,
    ) -> Option<Binary>;
}

pub struct Solution0;

#[async_trait]
impl Solution for Solution0 {
    async fn solve(repositories: Vec<ServerName>) -> Option<Binary> {
        let total_downloads = repositories.len() - 1;
        let (tx_binary, mut rx) = mpsc::channel::<Option<Binary>>(total_downloads);
        let (tx_cancel, _) = watch::channel(false);

        Self::spawn_all_downloads(repositories, tx_cancel.subscribe(), tx_binary).await;
        Self::receive_downloaded_data(&mut rx, tx_cancel, total_downloads).await
    }

    async fn download_task(
        repository: ServerName,
        mut cancel: watch::Receiver<bool>,
        tx_binary: mpsc::Sender<Option<Binary>>,
    ) {
        for attempt in 0..MAX_DOWNLOAD_ATTEMPTS {
            let result: Option<Binary> = tokio::select! {
                // If we got download finished first, then return result through channel
                binary = download(repository.clone()) => {
                    binary.ok()
                }

                // If we got cancel signal before download finished, then we cancel the task
                _ = cancel.changed() => {
                    return
                }
            };

            if result.is_some() {
                let _ = tx_binary.send(result).await;
                break;
            }
            println!(
                "The {} attempt of download is failed: {:?}",
                attempt + 1,
                repository
            );
        }
    }

    async fn spawn_all_downloads(
        repositories: Vec<ServerName>,
        rx_cancel: watch::Receiver<bool>,
        tx_binary: mpsc::Sender<Option<Binary>>,
    ) {
        tokio::spawn(async move {
            for repository in repositories {
                let tx_binary = tx_binary.clone();
                let rx_cancel = rx_cancel.clone();
                tokio::spawn(async move {
                    Self::download_task(repository.clone(), rx_cancel, tx_binary).await;
                });
            }
        });
    }

    async fn cancel_tasks(tx_cancel: &watch::Sender<bool>) {
        tx_cancel.send_modify(|cancel| {
            *cancel = true;
        });
    }

    async fn receive_downloaded_data(
        rx: &mut mpsc::Receiver<Option<Binary>>,
        tx_cancel: watch::Sender<bool>,
        total_downloads: usize,
    ) -> Option<Binary> {
        for _ in 0..=total_downloads {
            if let Some(binary) = rx.recv().await.unwrap_or(None) {
                // Some download is completed
                Self::cancel_tasks(&tx_cancel).await;
                return Some(binary);
            }
        }
        // Failed all downloads
        None
    }
}
