pub struct Dsp(Vec<(String, String)>);

impl Dsp {
    pub fn new(s: String) -> Dsp {
        let lines = s.lines();
        let mut v: Vec<(String, String)> = Vec::new();

        for line in lines {
            let a = line.split(':')
                .map(|a| a.to_string())
                .collect::<Vec<String>>();

            v.push((a[0].to_string(), a[1].to_string()));
        }

        Dsp(v)
    }

    pub fn get(&self, k: &String) -> Option<String> {
        for pair in &self.0 {
            if &pair.0 == k {
                return Some(pair.1.to_string());
            }
        }

        None
    }

    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();

        for p in &self.0 {
            keys.push(p.0.to_string());
        }

        keys
    }

    pub fn values(&self) -> Vec<String> {
        let mut values: Vec<String> = Vec::new();

        for p in &self.0 {
            values.push(p.0.to_string());
        }

        values
    }
}