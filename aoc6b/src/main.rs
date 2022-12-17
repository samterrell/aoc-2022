fn different(buffer: &[u8]) -> bool {
    if buffer.len() > 255 {
        return false;
    }
    buffer.iter().enumerate().all(|(i, b)| {
        buffer[0..i].iter().all(|b2| b2 != b) && buffer[i + 1..].iter().all(|b3| b3 != b)
    })
}

fn locate_signal<'a>(input: &'a str, window: usize) -> Option<usize> {
    Some(input.as_bytes().windows(window).position(different)? + window)
}

fn main() {
    let mut buff = String::new();
    std::io::stdin()
        .read_line(&mut buff)
        .expect("Couldn't read input");
    let result = locate_signal(&buff, 14).expect("No Signal");
    println!("{result}");
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(
            locate_signal("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            Some(19)
        );
        assert_eq!(locate_signal("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
        assert_eq!(locate_signal("nppdvjthqldpwncqszvftbrmjlhg", 14), Some(23));
        assert_eq!(
            locate_signal("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            Some(29)
        );
        assert_eq!(
            locate_signal("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            Some(26)
        );
    }
}
