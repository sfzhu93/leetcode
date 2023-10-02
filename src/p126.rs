use std::collections::VecDeque;
use std::collections::HashSet;

struct Dfs<'a> {
    depth: i32,
    end_node: usize,
    graph: &'a Vec<Vec<usize>>,
    visited: &'a mut HashSet<usize>,
    answer: &'a mut Vec<Vec<usize>>,
    stack: VecDeque<usize>,
}

impl Dfs<'_> {
    fn dfs(&mut self, i: usize, depth: i32) {
        if self.depth == depth {
            if i == self.end_node {
                self.answer.push(Vec::from(self.stack.clone()));
                println!("found answer");
            }
            println!("not an answer. stack = {:?}", self.stack);
        } else {
            for v in &self.graph[i] {
                if !self.visited.contains(v) {
                    let v_ = *v;
                    self.visited.insert(v_);
                    self.stack.push_back(v_);
                    self.dfs(v_, depth + 1);
                    self.stack.pop_back();
                    self.visited.remove(v);
                }
            }
        }
    }
}

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {

        // algorithm: bfs to find depth first, then apply dfs to find all answers.
        let n = word_list.len();
        let m = begin_word.len();
        let is_connected = |a: &[u8], b: &[u8]| -> bool {
            let mut ret = false;
            for i in 0..m {
                if a[i] != b[i] {
                    if !ret { ret = true; } else { return false; }
                }
            }
            return ret;
        };

        let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
        // index n is begin word
        for i in 0..n {
            for j in (i + 1)..n {
                if is_connected(word_list[i].as_bytes(), word_list[j].as_bytes()) {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }

        // connect begin words
        for i in 0..n {
            if is_connected(word_list[i].as_bytes(), begin_word.as_bytes()) {
                graph[i].push(n);
                graph[n].push(i);
            }
        }

        // start bfs
        let start_node = n; // word_list.iter().position(|s|s.eq(&begin_word)).unwrap();
        let end_node = match word_list.iter().position(|s| s.eq(&end_word)) {
            Some(n) => n,
            None => { return vec![]; }
        };
        let mut queue: VecDeque<(usize, i32)> = VecDeque::default(); // (graph node, step)
        let mut visited: HashSet<usize> = HashSet::default();

        let mut ans_size = -1 as i32;

        queue.push_front((start_node, 0));
        visited.insert(start_node);
        while !queue.is_empty() {
            let (node, node_size) = queue.pop_front().unwrap();
            if node == end_node {
                ans_size = node_size;
                break;
            }
            for v in &graph[node] {
                if !visited.contains(v) {
                    visited.insert(*v);
                    queue.push_back((*v, node_size + 1));
                }
            }
        }

        if ans_size == -1 {
            return vec![];
        }

        println!("ans_size = {}", ans_size);
        let mut visited: HashSet<usize> = HashSet::default();
        let mut answer: Vec<Vec<usize>> = vec![];
        visited.insert(start_node);
        let mut dfs: Dfs = Dfs {
            depth: ans_size,
            end_node,
            graph: &graph,
            visited: &mut visited,
            answer: &mut answer,
            stack: {
                let mut stack = VecDeque::default();
                stack.push_front(start_node);
                stack
            },
        };
        dfs.dfs(start_node, 0);
        return answer.iter().map(|x| x.iter().map(
            |n| if *n == start_node { begin_word.clone() } else { word_list[*n].clone() }
        ).collect()).collect::<Vec<Vec<String>>>();
    }
}

struct Solution;

use std::error::Error;

fn test(begin_word: &str, end_word: &str, word_list: Vec<&str>) -> Result<(), Box<dyn Error>> {
    // let word_list = serde_json::from_str(word_list_input)?;
    let word_list = word_list.iter().map(|x| x.to_string()).collect();
    println!("{:?}", Solution::find_ladders(begin_word.to_string(), end_word.to_string(), word_list));
    // assert_eq!(, result);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // test("qa", "sq", vec!["si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm", "le", "av", "sm", "ar", "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo", "ow", "sn", "ya", "cr", "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur", "rh", "sr", "tc", "lt", "lo", "as", "fr", "nb", "yb", "if", "pb", "ge", "th", "pm", "rb", "sh", "co", "ga", "li", "ha", "hz", "no", "bi", "di", "hi", "qa", "pi", "os", "uh", "wm", "an", "me", "mo", "na", "la", "st", "er", "sc", "ne", "mn", "mi", "am", "ex", "pt", "io", "be", "fm", "ta", "tb", "ni", "mr", "pa", "he", "lr", "sq", "ye"])?;
    test("cet", "ism", vec!["kid","tag","pup","ail","tun","woo","erg","luz","brr","gay","sip","kay","per","val","mes","ohs","now","boa","cet","pal","bar","die","war","hay","eco","pub","lob","rue","fry","lit","rex","jan","cot","bid","ali","pay","col","gum","ger","row","won","dan","rum","fad","tut","sag","yip","sui","ark","has","zip","fez","own","ump","dis","ads","max","jaw","out","btu","ana","gap","cry","led","abe","box","ore","pig","fie","toy","fat","cal","lie","noh","sew","ono","tam","flu","mgm","ply","awe","pry","tit","tie","yet","too","tax","jim","san","pan","map","ski","ova","wed","non","wac","nut","why","bye","lye","oct","old","fin","feb","chi","sap","owl","log","tod","dot","bow","fob","for","joe","ivy","fan","age","fax","hip","jib","mel","hus","sob","ifs","tab","ara","dab","jag","jar","arm","lot","tom","sax","tex","yum","pei","wen","wry","ire","irk","far","mew","wit","doe","gas","rte","ian","pot","ask","wag","hag","amy","nag","ron","soy","gin","don","tug","fay","vic","boo","nam","ave","buy","sop","but","orb","fen","paw","his","sub","bob","yea","oft","inn","rod","yam","pew","web","hod","hun","gyp","wei","wis","rob","gad","pie","mon","dog","bib","rub","ere","dig","era","cat","fox","bee","mod","day","apr","vie","nev","jam","pam","new","aye","ani","and","ibm","yap","can","pyx","tar","kin","fog","hum","pip","cup","dye","lyx","jog","nun","par","wan","fey","bus","oak","bad","ats","set","qom","vat","eat","pus","rev","axe","ion","six","ila","lao","mom","mas","pro","few","opt","poe","art","ash","oar","cap","lop","may","shy","rid","bat","sum","rim","fee","bmw","sky","maj","hue","thy","ava","rap","den","fla","auk","cox","ibo","hey","saw","vim","sec","ltd","you","its","tat","dew","eva","tog","ram","let","see","zit","maw","nix","ate","gig","rep","owe","ind","hog","eve","sam","zoo","any","dow","cod","bed","vet","ham","sis","hex","via","fir","nod","mao","aug","mum","hoe","bah","hal","keg","hew","zed","tow","gog","ass","dem","who","bet","gos","son","ear","spy","kit","boy","due","sen","oaf","mix","hep","fur","ada","bin","nil","mia","ewe","hit","fix","sad","rib","eye","hop","haw","wax","mid","tad","ken","wad","rye","pap","bog","gut","ito","woe","our","ado","sin","mad","ray","hon","roy","dip","hen","iva","lug","asp","hui","yak","bay","poi","yep","bun","try","lad","elm","nat","wyo","gym","dug","toe","dee","wig","sly","rip","geo","cog","pas","zen","odd","nan","lay","pod","fit","hem","joy","bum","rio","yon","dec","leg","put","sue","dim","pet","yaw","nub","bit","bur","sid","sun","oil","red","doc","moe","caw","eel","dix","cub","end","gem","off","yew","hug","pop","tub","sgt","lid","pun","ton","sol","din","yup","jab","pea","bug","gag","mil","jig","hub","low","did","tin","get","gte","sox","lei","mig","fig","lon","use","ban","flo","nov","jut","bag","mir","sty","lap","two","ins","con","ant","net","tux","ode","stu","mug","cad","nap","gun","fop","tot","sow","sal","sic","ted","wot","del","imp","cob","way","ann","tan","mci","job","wet","ism","err","him","all","pad","hah","hie","aim"])?;

    // test(2, "[[1,0],[0,1]]", false)?;
    Ok(())
}