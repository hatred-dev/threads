use std::sync::mpsc;
use std::thread;

//our threshold
const NUM_THREADS: usize = 6;
fn main() {
    let mut data = Vec::new();
    for i in 1..400_000_001_i32 {
        data.push(i)
    }
    let length = data.len();
    let result = split_on_threads(data, |data| (data as f64).log(3.33333333));
    assert!(length == result.len())
}

fn split_on_threads<T, R>(data: Vec<T>, f: fn(t: T) -> R) -> Vec<R>
where
    T: 'static + Sync + Send + Clone,
    R: 'static + Sync + Send,
{
    match data.len() {
        len if len <= NUM_THREADS => {
            let mut result = Vec::with_capacity(len);
            data.into_iter().for_each(|i| result.push(f(i)));
            result
        }
        len => {
            let (tx, rx) = mpsc::channel();
            let holder_size = len / NUM_THREADS;
            let mut threads = Vec::new();
            for chunk in data.chunks(holder_size).map(|chunk| chunk.to_owned()) {
                let thread_tx = tx.clone();
                threads.push(thread::spawn(move || {
                    thread_tx
                        .send(chunk.into_iter().map(f).collect::<Vec<R>>())
                        .unwrap()
                }));
            }

            let mut result = Vec::with_capacity(len);
            for _ in 0..NUM_THREADS + 1 {
                rx.recv()
                    .unwrap()
                    .into_iter()
                    .for_each(|data| result.push(data));
            }

            for thread in threads {
                thread.join().expect("Failet to join thread");
            }
            result
        }
    }
}
