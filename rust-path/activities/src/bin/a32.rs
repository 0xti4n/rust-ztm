// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Name<'a> {
    inner: Vec<&'a str>,
}

struct Titles<'a> {
    inner: Vec<&'a str>,
}

fn main() {
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();

    let names: Vec<_> = data.iter().filter_map(|x| x.split(',').nth(1)).collect();

    let names = Name { inner: names };

    let titles: Vec<_> = data.iter().filter_map(|x| x.split(',').nth(4)).collect();

    let titles = Titles { inner: titles };

    let info: Vec<_> = names.inner.iter().zip(titles.inner.iter()).collect();

    for (name, title) in info {
        println!("Name: {}, Title: {}", name, title);
    }
}