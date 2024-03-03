use std::collections::HashMap;
use std::fs;

type Patterns<'a> = HashMap<&'a [u8], Vec<usize>>;

fn compress(data: Vec<u8>) -> (Vec<u8>, HashMap<Vec<u8>, Vec<usize>>) {
    let mut compressed_data: Vec<u8> = data.clone();
    fn sliding_window<'a>(data: &'a [u8], window_size: usize, patterns: &mut Patterns<'a>) {
        let mut window = Vec::with_capacity(window_size);
        for (i, item) in data.iter().enumerate() {
            if i >= data.len() - window_size {
                break;
            }
            window.push(item);
            if window.len() > window_size {
                window.remove(0); // Remove oldest element when window is full
            }
            if window.len() == window_size {
                match patterns.get_mut(&data[i..i + window_size]) {
                    Some(pattern) => {
                        pattern.push(i);
                    }
                    None => {
                        patterns.insert(&data[i..i + window_size], vec![1]);
                    }
                }
            }
        }
    }
    let max_window_length = 20usize;
    let mut patterns: Patterns = HashMap::new();
    // stores the pattern as the key, and the list of indicies where the pattern occurs as the value

    sliding_window(&data, 3, &mut patterns);
    println!("{:?}", patterns);
    for (pattern, matches) in patterns {
        for match_ in matches {}
    }
    todo!()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let very_long_data = fs::read(&args[1]).expect("Cannot open file");
    println!("Original data len: {:?}", very_long_data.len());
    let (compressed_data, dictionary) = compress(very_long_data);
    println!("Compressed data len: {:?}", compressed_data.len());
    println!("Dictionary: {:?}", dictionary);
}
