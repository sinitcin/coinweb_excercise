use derive_more::Display;
use thiserror::Error;
use tokio::time;

#[derive(Error, Debug, Clone)]
pub enum ServerError {
    #[error("Server {0:?}: abruptly disconnected")]
    Disconnected(ServerName),
}

#[derive(Display, Debug, Clone)]
#[display(fmt = "Binary[source='{}']", "from.0")]
pub struct Binary {
    #[allow(dead_code)]
    from: ServerName,
}

#[derive(Debug, Clone)]
pub struct ServerName(pub String);

pub async fn download(server_name: ServerName) -> Result<Binary, ServerError> {
    let mut interval = time::interval(time::Duration::from_millis(100));
    for _i in 0..5 {
        interval.tick().await;
        if rand::random::<f32>() < 0.1 {
            return Err(ServerError::Disconnected(server_name.into()));
        }
    }
    Ok(Binary {
        from: server_name.into(),
    })
}
