import numpy as np
import matplotlib.pyplot as plt
import statistics as stats

# fn compute_fuel_cost_for_point(x: usize, crabs: &[usize]) -> usize {
#     crabs.iter().fold(0, |acc, crab| {
#         let n = crab.abs_diff(x);
#         acc + (n * (n + 1)) / 2
#     })
# }


def cost_for_point(x, crabs):
    s = 0
    for c in crabs:
        n = abs(c - x)
        s += (n * (n + 1)) / 2
    return s


if __name__ == '__main__':
    # data = np.array("16,1,2,0,4,2,7,1,2,14".split(',')).astype(int)
    data = np.random.randint(0,100,size=30).astype(int)
    # print(data)
    vals = np.array([cost_for_point(x, data) for x in range(data.min(), data.max())])
    sorted_indices = vals.argsort()
    real_root = sorted_indices[0]


    stat_fn = stats.mean
    print(data)
    conjecture_root = round(stat_fn(data))
    print(f"ans: {stat_fn(data)} -> {conjecture_root} (should be {real_root})")

    plt.plot(vals)
    plt.show()

    