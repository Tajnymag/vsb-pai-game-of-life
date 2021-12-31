from numpy.typing import NDArray

import numpy as np
from scipy.ndimage import convolve


def evolve_cells(board: NDArray[bool]) -> NDArray[bool]:
    kernel = [
        [1, 1, 1],
        [1, 0, 1],
        [1, 1, 1]
    ]

    neighbors = convolve(input=np.array(board).astype(int), weights=kernel, mode="constant", cval=0)

    where_alive = board
    where_neighbors_two_or_three = (neighbors == 2) | (neighbors == 3)
    where_dead = np.logical_not(board)
    where_neighbors_three = neighbors == 3
    survived = where_alive & where_neighbors_two_or_three
    resurrected = where_dead & where_neighbors_three
    evolved = survived | resurrected

    return evolved


