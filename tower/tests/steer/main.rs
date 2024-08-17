#![cfg(feature = "steer")]
#[path = "../support.rs"]
mod support;

use futures_util::future::{ready, Ready};
use tower::steer::Steer;
use tower_service::Service;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

struct MyService(u8);

impl Service<String> for MyService {
    type Response = u8;
    type Error = StdError;
    type Future = Ready<Result<u8, Self::Error>>;

    fn call(&mut self, _req: String) -> Self::Future {
        ready(Ok(self.0))
    }
}

#[tokio::test(flavor = "current_thread")]
async fn pick_correctly() {
    let _t = support::trace_init();
    let srvs = vec![MyService(42), MyService(57)];
    let mut st = Steer::new(srvs, |_: &_, _: &[_]| 1);

    let r = st.call(String::from("foo")).await.unwrap();
    assert_eq!(r, 57);
}
