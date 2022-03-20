mod macros;
use std::thread;

pub fn split_on_threads<T, R>(data: Vec<T>, func: fn(t: T) -> R, threshold: usize) -> Vec<R>
where
    T: 'static + Send + Clone,
    R: 'static + Send,
{
    match data.len() {
        len if len <= threshold => data.into_iter().map(func).collect(),
        len => {
            let mut threads = Vec::new();
            let mut result = Vec::with_capacity(len);
            for chunk in data
                .chunks(len / threshold + 1)
                .map(|chunk| chunk.to_owned())
            {
                threads.push(thread::spawn(move || {
                    chunk.into_iter().map(func).collect::<Vec<R>>()
                }));
            }
            for thread in threads {
                result.append(&mut thread.join().expect("Failed to join thread."));
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            duration!(split_on_threads(data, |x| (x as f64).log(3.33).log10(), 6));
        println!("Duration multithread {:?}", second_dur);
        assert!(first_dur > second_dur);
        assert!(first == second);
        assert!(first.len() == second.len())
    }
}
