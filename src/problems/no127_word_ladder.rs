pub struct Solution {}
impl Solution {
    pub fn find_valid_index(target: &str, word_list: &Vec<String>) -> Vec<usize> {
        let target = target.as_bytes();
        let mut res = Vec::new();
        for (i, word) in word_list.iter().enumerate() {
            let mut diff_count = 0;
            for (c1, c2) in target.iter().zip(word.as_bytes().iter()) {
                if c1 != c2 {
                    diff_count += 1;
                }
            }
            if diff_count <= 1 {
                res.push(i);
            }
        }
        res
    }

    pub fn gen_graph_map(graph: &mut Graph, word_list: &Vec<String>) {
        for (idx_word, word) in word_list.iter().enumerate() {
            let indexes = Solution::find_valid_index(word, word_list);
            for index in indexes {
                if word != &word_list[index] {
                    graph.insert_bidirection(idx_word, index);
                }
            }
        }
    }

    pub fn calculate_dist(graph: &Graph, sp: &String, ep: &String) -> Vec<usize> {
        let idx_sp = graph.index_list.iter().position(|v| v == sp).unwrap();
        let idx_ep = graph.index_list.iter().position(|v| v == ep).unwrap();
        let mut dist_table = vec![usize::MAX; graph.index_list.len()];

        let mut step = 0;
        dist_table[idx_sp] = step;
        let mut indexes = graph.maps[idx_sp].clone();
        let mut is_find = false;
        while !is_find && indexes.len() > 0 {
            step += 1;
            let mut next_indexes = Vec::new();
            for &i in &indexes {
                if i == idx_ep {
                    is_find = true;
                }
                if dist_table[i] > step {
                    dist_table[i] = step;
                    for &next_dest in graph.maps[i].iter() {
                        next_indexes.push(next_dest);
                    }
                }
            }
            indexes = next_indexes;
        }

        dist_table
    }

    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // begin_word may not in word_list and not present in return list
        // end_word need to present in word_list and return list

        //check end_word in list or not
        if let Some(idx_ep) = word_list.iter().position(|v| v==&end_word) {
            let mut word_list = word_list;
            if !word_list.contains(&begin_word) {
                word_list.push(begin_word.to_string());
            }
            let mut graph = Graph::new(&word_list);
    
            Solution::gen_graph_map(&mut graph, &word_list);
    
            let dist_table = Solution::calculate_dist(&graph, &begin_word, &end_word);
            (dist_table[idx_ep] + 1) as i32
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    index_list: Vec<String>,
    maps: Vec<Vec<usize>>, // start: end
}

impl Graph {
    pub fn new(members: &Vec<String>) -> Self {
        Self {
            index_list: members.clone(),
            maps: vec![Vec::new(); members.len()],
        }
    }

    pub fn insert_bidirection(&mut self, p1: usize, p2: usize) {
        self.insert_direction(p1, p2);
        self.insert_direction(p2, p1);
    }

    pub fn insert_direction(&mut self, sp: usize, ep: usize) {
        if !self.maps[sp].contains(&ep) {
            self.maps[sp].push(ep);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string(),
            ],
        );
        let except = 5;
        let output = Solution::ladder_length(inputs.0, inputs.1, inputs.2);
        assert_eq!(except, output);
    }
}