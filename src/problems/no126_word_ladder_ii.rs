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

    pub fn gen_shortest_paths(
        result: &mut Vec<Vec<String>>,
        mut path: Vec<String>,
        graph: &Graph,
        dist_table: &Vec<usize>,
        sp: &String,
        ep: &String,
        step: usize,
    ) {
        path.push(sp.clone());
        if sp == ep {
            result.push(path);
            return;
        }
        let idx_sp = graph.index_list.iter().position(|v| v == sp).unwrap();
        let idx_ep = graph.index_list.iter().position(|v| v == ep).unwrap();
        let indexes = &graph.maps[idx_sp];
        for &idx in indexes {
            if step == dist_table[idx] && step <=dist_table[idx_ep] {
                let next_sp = &graph.index_list[idx];
                 Solution::gen_shortest_paths(
                    result,
                    path.clone(),
                    graph,
                    dist_table,
                    next_sp,
                    ep,
                    step + 1,
                );
            }
        }
    }

    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
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
            // println!("{:#?}", graph);
    
            let mut dist_table = Solution::calculate_dist(&graph, &begin_word, &end_word);
            let dist_table_rev = Solution::calculate_dist(&graph, &end_word, &begin_word);
            let min_step = dist_table[idx_ep];
    
            // remove incorrect path
            for (idx, (&d1, &d2)) in dist_table.clone().iter().zip(dist_table_rev.iter()).enumerate() {
                if (d1.checked_add(d2).unwrap_or(usize::MAX) )!=min_step {
                    dist_table[idx] = usize::MAX;
                }
            }
    
            // println!("{:?}", dist_table);
            // println!("{:?}", dist_table_rev);
    
            let mut results = Vec::new();
            Solution::gen_shortest_paths(
                &mut results,
                Vec::new(),
                &graph,
                &dist_table,
                &begin_word,
                &end_word,
                1,
            );
    
            // println!("{:?}", results);
    
            results
        } else {
            vec![]
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
        let except = vec![
            vec![
                "hit".to_string(),
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "cog".to_string(),
            ],
            vec![
                "hit".to_string(),
                "hot".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string(),
            ],
        ];
        let output = Solution::find_ladders(inputs.0, inputs.1, inputs.2);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = (
            "hot".to_string(),
            "dog".to_string(),
            vec!["hot".to_string(), "dog".to_string()],
        );
        let except: Vec<Vec<String>> = Vec::new();
        let output = Solution::find_ladders(inputs.0, inputs.1, inputs.2);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = (
            "hot".to_string(),
            "dot".to_string(),
            vec!["hot".to_string(), "dot".to_string()],
        );
        let except: Vec<Vec<String>> = vec![vec!["hot".to_string(), "dot".to_string()]];
        let output = Solution::find_ladders(inputs.0, inputs.1, inputs.2);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = (
            "aaaaa".to_string(),
            "ggggg".to_string(),
            vec![
                "aaaaa".to_string(),
                "caaaa".to_string(),
                "cbaaa".to_string(),
                "daaaa".to_string(),
                "dbaaa".to_string(),
                "eaaaa".to_string(),
                "ebaaa".to_string(),
                "faaaa".to_string(),
                "fbaaa".to_string(),
                "gaaaa".to_string(),
                "gbaaa".to_string(),
                "haaaa".to_string(),
                "hbaaa".to_string(),
                "iaaaa".to_string(),
                "ibaaa".to_string(),
                "jaaaa".to_string(),
                "jbaaa".to_string(),
                "kaaaa".to_string(),
                "kbaaa".to_string(),
                "laaaa".to_string(),
                "lbaaa".to_string(),
                "maaaa".to_string(),
                "mbaaa".to_string(),
                "naaaa".to_string(),
                "nbaaa".to_string(),
                "oaaaa".to_string(),
                "obaaa".to_string(),
                "paaaa".to_string(),
                "pbaaa".to_string(),
                "bbaaa".to_string(),
                "bbcaa".to_string(),
                "bbcba".to_string(),
                "bbdaa".to_string(),
                "bbdba".to_string(),
                "bbeaa".to_string(),
                "bbeba".to_string(),
                "bbfaa".to_string(),
                "bbfba".to_string(),
                "bbgaa".to_string(),
                "bbgba".to_string(),
                "bbhaa".to_string(),
                "bbhba".to_string(),
                "bbiaa".to_string(),
                "bbiba".to_string(),
                "bbjaa".to_string(),
                "bbjba".to_string(),
                "bbkaa".to_string(),
                "bbkba".to_string(),
                "bblaa".to_string(),
                "bblba".to_string(),
                "bbmaa".to_string(),
                "bbmba".to_string(),
                "bbnaa".to_string(),
                "bbnba".to_string(),
                "bboaa".to_string(),
                "bboba".to_string(),
                "bbpaa".to_string(),
                "bbpba".to_string(),
                "bbbba".to_string(),
                "abbba".to_string(),
                "acbba".to_string(),
                "dbbba".to_string(),
                "dcbba".to_string(),
                "ebbba".to_string(),
                "ecbba".to_string(),
                "fbbba".to_string(),
                "fcbba".to_string(),
                "gbbba".to_string(),
                "gcbba".to_string(),
                "hbbba".to_string(),
                "hcbba".to_string(),
                "ibbba".to_string(),
                "icbba".to_string(),
                "jbbba".to_string(),
                "jcbba".to_string(),
                "kbbba".to_string(),
                "kcbba".to_string(),
                "lbbba".to_string(),
                "lcbba".to_string(),
                "mbbba".to_string(),
                "mcbba".to_string(),
                "nbbba".to_string(),
                "ncbba".to_string(),
                "obbba".to_string(),
                "ocbba".to_string(),
                "pbbba".to_string(),
                "pcbba".to_string(),
                "ccbba".to_string(),
                "ccaba".to_string(),
                "ccaca".to_string(),
                "ccdba".to_string(),
                "ccdca".to_string(),
                "cceba".to_string(),
                "cceca".to_string(),
                "ccfba".to_string(),
                "ccfca".to_string(),
                "ccgba".to_string(),
                "ccgca".to_string(),
                "cchba".to_string(),
                "cchca".to_string(),
                "cciba".to_string(),
                "ccica".to_string(),
                "ccjba".to_string(),
                "ccjca".to_string(),
                "cckba".to_string(),
                "cckca".to_string(),
                "cclba".to_string(),
                "cclca".to_string(),
                "ccmba".to_string(),
                "ccmca".to_string(),
                "ccnba".to_string(),
                "ccnca".to_string(),
                "ccoba".to_string(),
                "ccoca".to_string(),
                "ccpba".to_string(),
                "ccpca".to_string(),
                "cccca".to_string(),
                "accca".to_string(),
                "adcca".to_string(),
                "bccca".to_string(),
                "bdcca".to_string(),
                "eccca".to_string(),
                "edcca".to_string(),
                "fccca".to_string(),
                "fdcca".to_string(),
                "gccca".to_string(),
                "gdcca".to_string(),
                "hccca".to_string(),
                "hdcca".to_string(),
                "iccca".to_string(),
                "idcca".to_string(),
                "jccca".to_string(),
                "jdcca".to_string(),
                "kccca".to_string(),
                "kdcca".to_string(),
                "lccca".to_string(),
                "ldcca".to_string(),
                "mccca".to_string(),
                "mdcca".to_string(),
                "nccca".to_string(),
                "ndcca".to_string(),
                "occca".to_string(),
                "odcca".to_string(),
                "pccca".to_string(),
                "pdcca".to_string(),
                "ddcca".to_string(),
                "ddaca".to_string(),
                "ddada".to_string(),
                "ddbca".to_string(),
                "ddbda".to_string(),
                "ddeca".to_string(),
                "ddeda".to_string(),
                "ddfca".to_string(),
                "ddfda".to_string(),
                "ddgca".to_string(),
                "ddgda".to_string(),
                "ddhca".to_string(),
                "ddhda".to_string(),
                "ddica".to_string(),
                "ddida".to_string(),
                "ddjca".to_string(),
                "ddjda".to_string(),
                "ddkca".to_string(),
                "ddkda".to_string(),
                "ddlca".to_string(),
                "ddlda".to_string(),
                "ddmca".to_string(),
                "ddmda".to_string(),
                "ddnca".to_string(),
                "ddnda".to_string(),
                "ddoca".to_string(),
                "ddoda".to_string(),
                "ddpca".to_string(),
                "ddpda".to_string(),
                "dddda".to_string(),
                "addda".to_string(),
                "aedda".to_string(),
                "bddda".to_string(),
                "bedda".to_string(),
                "cddda".to_string(),
                "cedda".to_string(),
                "fddda".to_string(),
                "fedda".to_string(),
                "gddda".to_string(),
                "gedda".to_string(),
                "hddda".to_string(),
                "hedda".to_string(),
                "iddda".to_string(),
                "iedda".to_string(),
                "jddda".to_string(),
                "jedda".to_string(),
                "kddda".to_string(),
                "kedda".to_string(),
                "lddda".to_string(),
                "ledda".to_string(),
                "mddda".to_string(),
                "medda".to_string(),
                "nddda".to_string(),
                "nedda".to_string(),
                "oddda".to_string(),
                "oedda".to_string(),
                "pddda".to_string(),
                "pedda".to_string(),
                "eedda".to_string(),
                "eeada".to_string(),
                "eeaea".to_string(),
                "eebda".to_string(),
                "eebea".to_string(),
                "eecda".to_string(),
                "eecea".to_string(),
                "eefda".to_string(),
                "eefea".to_string(),
                "eegda".to_string(),
                "eegea".to_string(),
                "eehda".to_string(),
                "eehea".to_string(),
                "eeida".to_string(),
                "eeiea".to_string(),
                "eejda".to_string(),
                "eejea".to_string(),
                "eekda".to_string(),
                "eekea".to_string(),
                "eelda".to_string(),
                "eelea".to_string(),
                "eemda".to_string(),
                "eemea".to_string(),
                "eenda".to_string(),
                "eenea".to_string(),
                "eeoda".to_string(),
                "eeoea".to_string(),
                "eepda".to_string(),
                "eepea".to_string(),
                "eeeea".to_string(),
                "ggggg".to_string(),
                "agggg".to_string(),
                "ahggg".to_string(),
                "bgggg".to_string(),
                "bhggg".to_string(),
                "cgggg".to_string(),
                "chggg".to_string(),
                "dgggg".to_string(),
                "dhggg".to_string(),
                "egggg".to_string(),
                "ehggg".to_string(),
                "fgggg".to_string(),
                "fhggg".to_string(),
                "igggg".to_string(),
                "ihggg".to_string(),
                "jgggg".to_string(),
                "jhggg".to_string(),
                "kgggg".to_string(),
                "khggg".to_string(),
                "lgggg".to_string(),
                "lhggg".to_string(),
                "mgggg".to_string(),
                "mhggg".to_string(),
                "ngggg".to_string(),
                "nhggg".to_string(),
                "ogggg".to_string(),
                "ohggg".to_string(),
                "pgggg".to_string(),
                "phggg".to_string(),
                "hhggg".to_string(),
                "hhagg".to_string(),
                "hhahg".to_string(),
                "hhbgg".to_string(),
                "hhbhg".to_string(),
                "hhcgg".to_string(),
                "hhchg".to_string(),
                "hhdgg".to_string(),
                "hhdhg".to_string(),
                "hhegg".to_string(),
                "hhehg".to_string(),
                "hhfgg".to_string(),
                "hhfhg".to_string(),
                "hhigg".to_string(),
                "hhihg".to_string(),
                "hhjgg".to_string(),
                "hhjhg".to_string(),
                "hhkgg".to_string(),
                "hhkhg".to_string(),
                "hhlgg".to_string(),
                "hhlhg".to_string(),
                "hhmgg".to_string(),
                "hhmhg".to_string(),
                "hhngg".to_string(),
                "hhnhg".to_string(),
                "hhogg".to_string(),
                "hhohg".to_string(),
                "hhpgg".to_string(),
                "hhphg".to_string(),
                "hhhhg".to_string(),
                "ahhhg".to_string(),
                "aihhg".to_string(),
                "bhhhg".to_string(),
                "bihhg".to_string(),
                "chhhg".to_string(),
                "cihhg".to_string(),
                "dhhhg".to_string(),
                "dihhg".to_string(),
                "ehhhg".to_string(),
                "eihhg".to_string(),
                "fhhhg".to_string(),
                "fihhg".to_string(),
                "ghhhg".to_string(),
                "gihhg".to_string(),
                "jhhhg".to_string(),
                "jihhg".to_string(),
                "khhhg".to_string(),
                "kihhg".to_string(),
                "lhhhg".to_string(),
                "lihhg".to_string(),
                "mhhhg".to_string(),
                "mihhg".to_string(),
                "nhhhg".to_string(),
                "nihhg".to_string(),
                "ohhhg".to_string(),
                "oihhg".to_string(),
                "phhhg".to_string(),
                "pihhg".to_string(),
                "iihhg".to_string(),
                "iiahg".to_string(),
                "iiaig".to_string(),
                "iibhg".to_string(),
                "iibig".to_string(),
                "iichg".to_string(),
                "iicig".to_string(),
                "iidhg".to_string(),
                "iidig".to_string(),
                "iiehg".to_string(),
                "iieig".to_string(),
                "iifhg".to_string(),
                "iifig".to_string(),
                "iighg".to_string(),
                "iigig".to_string(),
                "iijhg".to_string(),
                "iijig".to_string(),
                "iikhg".to_string(),
                "iikig".to_string(),
                "iilhg".to_string(),
                "iilig".to_string(),
                "iimhg".to_string(),
                "iimig".to_string(),
                "iinhg".to_string(),
                "iinig".to_string(),
                "iiohg".to_string(),
                "iioig".to_string(),
                "iiphg".to_string(),
                "iipig".to_string(),
                "iiiig".to_string(),
                "aiiig".to_string(),
                "ajiig".to_string(),
                "biiig".to_string(),
                "bjiig".to_string(),
                "ciiig".to_string(),
                "cjiig".to_string(),
                "diiig".to_string(),
                "djiig".to_string(),
                "eiiig".to_string(),
                "ejiig".to_string(),
                "fiiig".to_string(),
                "fjiig".to_string(),
                "giiig".to_string(),
                "gjiig".to_string(),
                "hiiig".to_string(),
                "hjiig".to_string(),
                "kiiig".to_string(),
                "kjiig".to_string(),
                "liiig".to_string(),
                "ljiig".to_string(),
                "miiig".to_string(),
                "mjiig".to_string(),
                "niiig".to_string(),
                "njiig".to_string(),
                "oiiig".to_string(),
                "ojiig".to_string(),
                "piiig".to_string(),
                "pjiig".to_string(),
                "jjiig".to_string(),
                "jjaig".to_string(),
                "jjajg".to_string(),
                "jjbig".to_string(),
                "jjbjg".to_string(),
                "jjcig".to_string(),
                "jjcjg".to_string(),
                "jjdig".to_string(),
                "jjdjg".to_string(),
                "jjeig".to_string(),
                "jjejg".to_string(),
                "jjfig".to_string(),
                "jjfjg".to_string(),
                "jjgig".to_string(),
                "jjgjg".to_string(),
                "jjhig".to_string(),
                "jjhjg".to_string(),
                "jjkig".to_string(),
                "jjkjg".to_string(),
                "jjlig".to_string(),
                "jjljg".to_string(),
                "jjmig".to_string(),
                "jjmjg".to_string(),
                "jjnig".to_string(),
                "jjnjg".to_string(),
                "jjoig".to_string(),
                "jjojg".to_string(),
                "jjpig".to_string(),
                "jjpjg".to_string(),
                "jjjjg".to_string(),
                "ajjjg".to_string(),
                "akjjg".to_string(),
                "bjjjg".to_string(),
                "bkjjg".to_string(),
                "cjjjg".to_string(),
                "ckjjg".to_string(),
                "djjjg".to_string(),
                "dkjjg".to_string(),
                "ejjjg".to_string(),
                "ekjjg".to_string(),
                "fjjjg".to_string(),
                "fkjjg".to_string(),
                "gjjjg".to_string(),
                "gkjjg".to_string(),
                "hjjjg".to_string(),
                "hkjjg".to_string(),
                "ijjjg".to_string(),
                "ikjjg".to_string(),
                "ljjjg".to_string(),
                "lkjjg".to_string(),
                "mjjjg".to_string(),
                "mkjjg".to_string(),
                "njjjg".to_string(),
                "nkjjg".to_string(),
                "ojjjg".to_string(),
                "okjjg".to_string(),
                "pjjjg".to_string(),
                "pkjjg".to_string(),
                "kkjjg".to_string(),
                "kkajg".to_string(),
                "kkakg".to_string(),
                "kkbjg".to_string(),
                "kkbkg".to_string(),
                "kkcjg".to_string(),
                "kkckg".to_string(),
                "kkdjg".to_string(),
                "kkdkg".to_string(),
                "kkejg".to_string(),
                "kkekg".to_string(),
                "kkfjg".to_string(),
                "kkfkg".to_string(),
                "kkgjg".to_string(),
                "kkgkg".to_string(),
                "kkhjg".to_string(),
                "kkhkg".to_string(),
                "kkijg".to_string(),
                "kkikg".to_string(),
                "kkljg".to_string(),
                "kklkg".to_string(),
                "kkmjg".to_string(),
                "kkmkg".to_string(),
                "kknjg".to_string(),
                "kknkg".to_string(),
                "kkojg".to_string(),
                "kkokg".to_string(),
                "kkpjg".to_string(),
                "kkpkg".to_string(),
                "kkkkg".to_string(),
                "ggggx".to_string(),
                "gggxx".to_string(),
                "ggxxx".to_string(),
                "gxxxx".to_string(),
                "xxxxx".to_string(),
                "xxxxy".to_string(),
                "xxxyy".to_string(),
                "xxyyy".to_string(),
                "xyyyy".to_string(),
                "yyyyy".to_string(),
                "yyyyw".to_string(),
                "yyyww".to_string(),
                "yywww".to_string(),
                "ywwww".to_string(),
                "wwwww".to_string(),
                "wwvww".to_string(),
                "wvvww".to_string(),
                "vvvww".to_string(),
                "vvvwz".to_string(),
                "avvwz".to_string(),
                "aavwz".to_string(),
                "aaawz".to_string(),
                "aaaaz".to_string(),
            ],
        );
        let except: Vec<Vec<String>> = vec![vec![
            "aaaaa".to_string(),
            "aaaaz".to_string(),
            "aaawz".to_string(),
            "aavwz".to_string(),
            "avvwz".to_string(),
            "vvvwz".to_string(),
            "vvvww".to_string(),
            "wvvww".to_string(),
            "wwvww".to_string(),
            "wwwww".to_string(),
            "ywwww".to_string(),
            "yywww".to_string(),
            "yyyww".to_string(),
            "yyyyw".to_string(),
            "yyyyy".to_string(),
            "xyyyy".to_string(),
            "xxyyy".to_string(),
            "xxxyy".to_string(),
            "xxxxy".to_string(),
            "xxxxx".to_string(),
            "gxxxx".to_string(),
            "ggxxx".to_string(),
            "gggxx".to_string(),
            "ggggx".to_string(),
            "ggggg".to_string(),
        ]];
        let output = Solution::find_ladders(inputs.0, inputs.1, inputs.2);
        assert_eq!(except, output);
    }

    #[test]
    fn case5() {
        let inputs = (
            "hit".to_string(),
            "cog".to_string(),
            vec!["hot".to_string(),"dot".to_string(),"dog".to_string(),"lot".to_string(),"log".to_string()],
        );
        let except: Vec<Vec<String>> = vec![];
        let output = Solution::find_ladders(inputs.0, inputs.1, inputs.2);
        assert_eq!(except, output);
    }
}
