pub fn encode(source: &str) -> String {
    let mut now_ch = '\0';
    let mut counter = 1;
    let mut encoded = String::new();

    for (i, ch) in source
        .chars()
        .filter(|x| x.is_alphabetic() || x.is_whitespace())
        .enumerate()
    {
        if i == 0 {
            // initialize
            now_ch = ch;
        } else {
            // target_ch
            // check ch == now_ch
            // if ch == now_ch, then counter += 1
            // if ch != now_ch, then counter = 1 and now_ch = ch
            if ch == now_ch {
                counter += 1;
            } else {
                if counter > 1 {
                    encoded.push_str(&counter.to_string());
                }
                encoded.push_str(&now_ch.to_string());
                now_ch = ch;
                counter = 1;
            }
        }
        //finalize
        if i == source.len() - 1 {
            if counter > 1 {
                encoded.push_str(&counter.to_string());
            }
            encoded.push_str(&now_ch.to_string());
        }
    }
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut now_ch = '\0';
    let mut counter: String = "".to_string();

    for ch in source.chars() {
        if ch.is_alphabetic() || ch.is_whitespace() {
            now_ch = ch;
            if counter == "" {
                counter = "1".to_string();
            }
            decoded.push_str(&now_ch.to_string().repeat(counter.parse::<usize>().unwrap()));
            counter = "".to_string();
        } else {
            counter.push_str(&ch.to_string());
        }
    }
    decoded
}