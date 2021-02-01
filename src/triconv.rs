const START_OFFSET: u8 = b'a';
const ILLEGAL_LETTERS: &str = "'.,:;-_?!0123456789";

pub fn text_to_trinary(input: String) -> Vec<[u8; 3]> {

    let lc = input.to_lowercase()
        .chars()
        .filter(|x| !ILLEGAL_LETTERS.contains(*x))
        .map(|x| if x == '\n' {' '} else {x})
        .collect::<String>();

    lc.as_bytes()
        .into_iter()
        .map(|x| if *x == b' ' {26u8} else {x - START_OFFSET})
        .map(|x| {
            let mut trinum: [u8; 3] = [0, 0, 0];
            let mut buf = x;
            for i in 0..3 {
                let exp = 3u8.pow((trinum.len() - 1 - i) as u32);
                let t = buf / exp;
                trinum[i] = t;
                buf -= t * exp;
            }
            trinum 
        })
        .collect::<Vec<[u8; 3]>>()

}