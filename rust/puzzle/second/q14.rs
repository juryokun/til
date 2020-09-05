fn main() {
    // データセット
    let data = vec!["Brazil", "Croatia"];

    // // 国名を大文字変換
    // let upper_data = data
    //     .iter()
    //     .map(|st| st.chars().map(|c| c.to_ascii_uppercase()));

    // フラグ付与
    let mut target: Vec<Country> = vec![];
    for countory in data.iter() {
        target.push(Country::new(countory.to_string()));
    }

    // しりとり開始
}

#[derive(Debug, Clone, PartialEq)]
struct Country {
    name: String,
    is_used: bool,
}
impl Country {
    fn new(name: String) -> Self {
        let upper_name = name.chars().map(|c| c.to_ascii_uppercase()).collect();
        Self {
            name: upper_name,
            is_used: false,
        }
    }
    fn used(&self) -> Self {
        Self {
            name: self.name.clone(),
            is_used: true,
        }
    }
    fn is_used(&self) -> bool {
        self.is_used
    }
    fn equal_end(&self, c: &char) -> bool {
        &self.name.chars().last().unwrap() == c
    }
}

// TODO: カウント方法修正
fn shiritori(last_char: char, countries: Vec<Country>, mut cnt: i32) -> i32 {
    let is_candidate = |country: &Country| country.equal_end(&last_char) && country.is_used();
    for (index, val) in countries.iter().enumerate() {
        if is_candidate(val) {
            let mut new_countries = countries.clone();
            new_countries[index] = val.used();
            cnt = shiritori(
                val.name.chars().last().unwrap(),
                new_countries,
                cnt.clone() + 1,
            );
        }
    }
    cnt
}
