fn different(buffer: &[u8]) -> bool {
    if buffer.len() > 255 {
        return false;
    }
    buffer.iter().enumerate().all(|(i, b)| {
        buffer[0..i].iter().all(|b2| b2 != b) && buffer[i + 1..].iter().all(|b3| b3 != b)
    })
}

fn locate_signal<'a>(input: &'a str) -> Option<usize> {
    Some(input.as_bytes().windows(4).position(different)? + 4)
}

fn main() {
    let mut buff = String::new();
    std::io::stdin()
        .read_line(&mut buff)
        .expect("Couldn't read input");
    let result = locate_signal(&buff).expect("No Signal");
    println!("{result}");
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(locate_signal("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(locate_signal("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(locate_signal("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(locate_signal("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(locate_signal("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }
}
