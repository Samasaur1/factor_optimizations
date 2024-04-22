fn main() {
    fn factor_exists(num: &usize, pool: &Vec<usize>) -> bool {
        for i in pool {
            if num != i && num % i == 0 {
                return true
            }
        }
        return false
    }
    fn remove_factors(num: &usize, pool: &Vec<usize>) -> Vec<usize> {
        let mut new = vec![];
        for i in pool {
            if num % i != 0 {
                new.push(*i);
            }
        }
        return new;
    }
    fn solve(pool: &Vec<usize>) -> Vec<Vec<usize>> {
        if pool.len() <= 1 { return vec![vec![]]; }

        let mut res: Vec<Vec<usize>> = vec![vec![]];
        for i in pool {
            if !factor_exists(i, pool) {
                continue;
            }
            let new_pool = remove_factors(i, pool);
            let results = solve(&new_pool);

            for mut r in results {
                r.insert(0, *i);
                res.push(r);
            }
            // let results_: Vec<Vec<usize>> = results.iter().map(|mut x| x.insert(0, *i)).collect();
            // res.append(results_);
        }
        return res
    }

    let _solutions = solve(&vec![1usize,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24]);
    // let best: usize = _solutions.iter().map(|sol| sol.iter().sum()).max().unwrap();
}
