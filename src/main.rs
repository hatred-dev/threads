use std::sync::mpsc;
use std::thread;

//our threshold
const NUM_THREADS: usize = 6;

fn main() {
    let mut data = Vec::new();
    for i in 1..400_000_001_u32 {
        data.push(i)
    }
    let length = data.len();
    let result = split_on_threads(data, |data| (data as f64).log(3.33333333));
    println!("Size {}. Same: {}", result.len(), length == result.len());
}

fn split_on_threads<T, R>(data: Vec<T>, f: fn(t: T) -> R) -> Vec<R>
where
    T: 'static + Sync + Send + Clone,
    R: 'static + Sync + Send,
{
    match data.len() {
        len if len <= NUM_THREADS => {
            //memory preallocation makes program a bit faster
            let mut result = Vec::with_capacity(len);
            data.into_iter().for_each(|i| result.push(f(i)));
            result
        }
        len => {
            let (send, rec) = mpsc::channel();
            let holder_size = len / NUM_THREADS;
            for chunk in data.chunks(holder_size).map(|chunk| chunk.to_owned()) {
                let thread_send = send.clone();
                thread::spawn(move || {
                    thread_send
                        .send(chunk.into_iter().map(f).collect::<Vec<R>>())
                        .unwrap()
                });
            }

            let mut result = Vec::with_capacity(len);
            for _ in 0..NUM_THREADS + 1 {
                rec.recv()
                    .unwrap()
                    .into_iter()
                    .for_each(|data| result.push(data));
            }
            result
        }
    }
}
