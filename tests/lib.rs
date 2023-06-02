use rdefer::Defer;
use std::sync::Arc;

#[test]
fn test_defer() {
    let mut value = 0;
    {
        let _d = Defer::new(|| value = 1);
    }
    assert_eq!(value, 1);
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_async_defer() {
    use rdefer::{async_defer, exec_before_defer};
    use std::sync::{Arc, Mutex};

    let value = Arc::new(Mutex::new(0));
    let value_clone1 = Arc::clone(&value);
    let value_clone2 = Arc::clone(&value);

    let defer = async_defer!(2, async {
        // After the counter has been decremented twice, this will increment the value by 1.
        let mut value = value.lock().unwrap();
        *value += 1;
    });

    exec_before_defer!(defer, || {
        // This will increment the value by 1.
        let mut value = value_clone1.lock().unwrap();
        *value += 1;
    });

    exec_before_defer!(defer, || {
        // This will increment the value by 1 again.
        let mut value = value_clone2.lock().unwrap();
        *value += 1;
    });

    // Sleep here to allow async tasks to finish
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // At this point, the value should be 3.
    assert_eq!(*value.lock().unwrap(), 3);
}
