import numpy as np
import matplotlib.pyplot as plt


if __name__ == '__main__':
    x = np.array([16,1,2,0,4,2,7,1,2,14])
    y = []

    for n in range(0, max(x)):
        s = 0

        for xi in x:

            s += abs(xi - n)
        y.append(s)

    y = np.array(y)
    plt.plot(list(range(0, max(x))), y)
    plt.show()
