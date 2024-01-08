use async_trait::async_trait;

/// 数据处理-接收数据
#[async_trait]
pub trait Receive<P, R> {
    /// match_str：接收数据的路径，paramters：接收数据过程中携带的参数
    async fn receive(&mut self, match_str: String, paramters: P) -> R;
}

/// 数据处理-序列化数据为目标格式
pub trait Serde {
    type Target;

    fn serde(&mut self) -> Self::Target;
}

/// 数据处理-导出数据到目标平台
pub trait Export<R> {
    fn export(&mut self) -> R;
}
