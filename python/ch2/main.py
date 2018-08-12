from typing import List


def maximum_profit(n: int, arr: List[int]) -> int:
    maxv = -20000000
    minv = arr[0]
    for n in arr:
        maxv = max(maxv, n - minv)
        minv = min(minv, n)
    return maxv
