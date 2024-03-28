use async_std::{self, task::JoinHandle};

pub async fn create_node<F, Fut>(node:F)->JoinHandle<()>
where
    F:FnOnce() ->Fut + Send + 'static,
    Fut: async_std::future::Future<Output = ()> + Send + 'static,
{
    async_std::task::spawn(node())
}