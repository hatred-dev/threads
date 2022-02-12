use std::thread;

#[allow(dead_code)]
//our threshold
const NUM_THREADS: usize = 6;

fn main() {}

#[allow(dead_code)]
fn split_on_threads<T, R>(data: Vec<T>, func: fn(t: T) -> R) -> Vec<R>
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
            let holder_size = len / NUM_THREADS + 1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use thread_splitter::duration;

    #[test]
    fn test_identity_of_data() {
        let size = 500_000_000_i32;
        let mut data = Vec::with_capacity(size as usize);
        for i in 1..size + 1 {
            data.push(i)
        }
        let (first, first_dur) = duration!(data
            .iter()
            .map(|x| (*x as f64).log(3.33).log10())
            .collect::<Vec<f64>>());
        println!("Duration single thread {:?}", first_dur);
        let (second, second_dur) =
            duration!(split_on_threads(data, |x| (x as f64).log(3.33).log10()));
        println!("Duration multithread {:?}", second_dur);
        assert!(first_dur > second_dur);
        assert!(first == second);
        assert!(first.len() == second.len())
    }
}
