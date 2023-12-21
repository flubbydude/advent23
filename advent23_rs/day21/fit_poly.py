import numpy as np

if __name__ == '__main__':
    quad = np.polyfit([65, 327, 589, 851],
                      [-99, 33, 165, 297], 3)  # 132 * x

    print(f"{quad[0]}x^3 + {quad[1]}x^2 + {quad[2]}x + {quad[3]}")
