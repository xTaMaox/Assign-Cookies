impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut gh = BinaryHeap::with_capacity(g.len());
        let mut sh = BinaryHeap::with_capacity(s.len());
        g.iter().for_each(|&x| gh.push(Reverse(x)));
        s.iter().for_each(|&x| sh.push(Reverse(x)));

        let mut count = 0;
        while !gh.is_empty() && !sh.is_empty() {
            let g_val = gh.peek().unwrap().0;
            let s_val = sh.peek().unwrap().0;
           
            if s_val >= g_val {
                count += 1;
                gh.pop();
            }
            sh.pop();
        }
        count
    }
}