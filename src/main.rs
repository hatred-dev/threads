use std::thread;

//our threshold
const NUM_THREADS: usize = 6;

fn main() {
    let func = |x| x + 69;
    let mut data = Vec::new();
    for i in 1..500_000_001_i32 {
        data.push(i)
    }
    let first = data.clone().into_iter().map(func).collect::<Vec<i32>>();
    let second = split_on_threads(data, func);
    assert!(first == second);
    println!("{:?}", second.len());
}

fn split_on_threads<'a, T, R>(data: Vec<T>, func: fn(t: T) -> R) -> Vec<R>
where
    T: 'static + Sync + Send + Clone,
    R: 'static + Sync + Send,
{
    match data.len() {
        len if len <= NUM_THREADS => {
            //memory preallocation makes program a bit faster
            let mut result = Vec::with_capacity(len);
            data.into_iter().for_each(|i| result.push(func(i)));
            result
        }
        len => {
            let holder_size = len / NUM_THREADS;
            let mut threads = Vec::new();
            let mut result = Vec::with_capacity(len);
            for chunk in data.chunks(holder_size).map(|chunk| chunk.to_owned()) {
                threads.push(thread::spawn(move || {
                    chunk.into_iter().map(func).collect::<Vec<R>>()
                }));
            }
            for thread in threads {
                result.append(&mut thread.join().unwrap());
            }
            result
        }
    }
}
