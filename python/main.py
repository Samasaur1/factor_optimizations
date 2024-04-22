from functools import cache


def factor_exists(num: int, pool: set[int]) -> bool:
    for i in pool:
        if (num != i and num % i == 0):
            return True
    return False


def remove_factors(num: int, pool: set[int]) -> set[int]:
    return frozenset({i for i in pool if num % i != 0})


def solve(pool: set[int]) -> int:    
    best = 0
    for i in pool:
        if not factor_exists(i, pool):
            continue
        newPool = remove_factors(i, pool)
        result = solve(newPool)
        best = max(best, result + i)
    return best


solve(frozenset({x for x in range(1,24)}))
# print(solve(frozenset({x for x in range(1, 26)})))
