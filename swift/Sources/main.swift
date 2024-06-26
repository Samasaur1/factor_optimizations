let FACTOR_N = 2

private func factorExists(num: Int, pool: [Int]) -> Bool {
    for i in pool {
        if (num != i && num % i == 0) {
            return true
        }
    }
    return false
}
private func removeFactors(num: Int, pool: [Int]) -> [Int] {
    var new: [Int] = []
    for i in pool {
        if (num % i != 0) {
            new.append(i)
        }
    }
    return new
}

func solve(pool: [Int]) -> [[Int]] {
    guard !pool.isEmpty else { return [[]] }
    if pool.count == 1 { return [[]] }
    
    var res: [[Int]] = [[]]
    for i in pool {
        guard factorExists(num: i, pool: pool) else { continue }
        let newPool = removeFactors(num: i, pool: pool)
        let results = solve(pool: newPool)
        res.append(contentsOf: results.map { CollectionOfOne(i) + $0 })
    }
    return res
}

let _ = solve(pool: Array(1...FACTOR_N))
