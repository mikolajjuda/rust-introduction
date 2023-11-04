use std::future::Future;

async fn some_async_fn() -> u32 {
    5
}

async fn another_async_fn() -> u32 {
    some_async_fn().await + 1
}

// this is a hack to get a noop RawWaker
fn make_raw_waker() -> std::task::RawWaker {
    std::task::RawWaker::new(
        std::ptr::null(),
        &std::task::RawWakerVTable::new(|_| make_raw_waker(), |_| {}, |_| {}, |_| {}),
    )
}

fn main() {
    let future = async { another_async_fn().await * 2 };

    let result = {
        // this part below should be done by the executor
        let mut pinned_future = Box::pin(future);
        let waker = unsafe { std::task::Waker::from_raw(make_raw_waker()) };
        let mut context = &mut std::task::Context::from_waker(&waker);
        loop {
            match pinned_future.as_mut().poll(&mut context) {
                std::task::Poll::Ready(result) => break result,
                std::task::Poll::Pending => {
                    println!("not ready yet");
                }
            }
        }
        // our implementation of the executor is horrible
        // use something like pollster for minimal executor that actually works correctly
    };

    println!("Result: {}", result);
}
