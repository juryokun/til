fn main() {
    // データセット
    let data = vec![
        "Brazil",
        "Croatia",
        "Mexico",
        "Cameroon",
        "Spain",
        "Netherlands",
        "Chile",
        "Australia",
        "Colombia",
        "Greece",
        "Cote d'lvoire",
        "Japan",
        "Uruguay",
        "Costa Rica",
        "England",
        "Italy",
        "Switzerland",
        "Ecuador",
        "France",
        "Honduras",
        "Argentina",
        "Bosnia and Herzegovina",
        "Iran",
        "Nigeria",
        "Germany",
        "Portugal",
        "Ghana",
        "USA",
        "Belgium",
        "Algeria",
        "Russia",
        "Korea Republic",
    ];

    // フラグ付与
    let mut target: Vec<Country> = vec![];
    for countory in data.iter() {
        target.push(Country::new(countory.to_string()));
    }

    // しりとり開始
    let rel = shiritori(' ', target, 0);
    println!("{}", rel);
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
    fn equal_head(&self, c: &char) -> bool {
        &self.name.chars().next().unwrap() == c
    }
}

fn shiritori(last_char: char, countries: Vec<Country>, cnt: i32) -> i32 {
    let mut max_cnt = cnt;
    let is_candidate = |country: &Country| country.equal_head(&last_char) && !country.is_used();
    for (index, val) in countries.iter().enumerate() {
        if is_candidate(val) || last_char == ' ' {
            let mut new_countries = countries.clone();
            new_countries[index] = val.used();
            let tmp_cnt = shiritori(
                val.name.chars().last().unwrap(),
                new_countries,
                cnt.clone() + 1,
            );
            if max_cnt < tmp_cnt {
                max_cnt = tmp_cnt;
            }
        }
    }
    max_cnt
}

#[test]
fn test_country_new() {
    let check = Country {
        name: "ABC".to_string(),
        is_used: false,
    };
    assert_eq!(Country::new("Abc".to_string()), check);
}
#[test]
fn test_shiritori() {
    let data = vec!["Abc", "Bde", "Cab"];

    let mut target: Vec<Country> = vec![];
    for countory in data.iter() {
        target.push(Country::new(countory.to_string()));
    }

    // しりとり開始
    assert_eq!(shiritori(' ', target, 0), 3);
}
