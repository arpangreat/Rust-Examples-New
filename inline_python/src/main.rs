use inline_python::python;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

fn main() {
    let mut data = Vec::new();

    for _ in 0..10000 {
        let start = Instant::now();
        sleep(Duration::from_millis(1));
        let duration = start.elapsed();
        data.push(duration.as_secs_f64() * 1e6);
    }

    python! {
            import matplotlib.pyplot as plt

            plt.suptitle("Time taken to sleep 1 millisecond")

            plt.subplot(2, 1, 1)
            plt.title("measurement")
            plt.plot('data, ".")
            plt.ylabel("us")

            plt.subplot(2, 1, 2)
            plt.title("histogram")
            plt.hist('data, bins=250)
            plt.xlabel("us")

            plt.show()
    }
}
