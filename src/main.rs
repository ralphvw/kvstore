use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("Please enter a key");
    let value = args.next().expect("Please enter a value");

    let mut db = Database::new().unwrap();
    db.insert(key, value);
    db.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let contents = std::fs::read_to_string("kv.db")?;

        let mut map = HashMap::new();
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(self) -> Result<(), std::io::Error> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(&format!("{}\t{}\n", key, value));
        }
        std::fs::write("kv.db", contents)
    }
}
