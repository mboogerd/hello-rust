// Exercises mentioned in Summary of chapter 8
mod summary {

    pub fn average(ints: &Vec<i32>) -> Option<f64> {
        match ints.len() {
            0 => None,
            1 => Some(ints[0].into()),
            _ => {
                let sum = ints.iter().fold(0, |sum, x| sum + x) as f64;
                let count = ints.len() as f64;
                Some(sum / count)
            }
        }
    }

    pub fn median(ints: &mut Vec<i32>) -> Option<f64> {
        match ints.len() {
            0 => None,
            n => {
                ints.sort();
                match n % 2 {
                    0 => {
                        let fstHalf = ints.len() / 2 - 1;
                        let sndHalf = ints.len() - fstHalf;
                        let medianFragment = Vec::from(&ints[fstHalf..sndHalf]);
                        average(&medianFragment)
                    },
                    1 => {
                        let half = ints.len() / 2;
                        Some(ints[half].into())
                    }
                    _ => None
                }
            }
        }
    }

    pub fn mode(ints: &Vec<i32>) -> Option<i32> {
        use std::collections::HashMap;
    
        let mut map: HashMap<i32, u32> = HashMap::new();
        for i in ints.iter() {
            // Fetch the values' mutable reference for the word
            let count = map.entry(*i).or_insert(0);
            *count += 1;
        }

        let maxEntry = map.iter().max_by_key(|&(_, count)| count);
        maxEntry.map(|x| *x.0)
    }

    // TODO: Convert strings to pig latin
    // TODO: Create a text interface to allow a user to add
    //       employee names to a department in a company
}

#[cfg(test)]
mod tests;