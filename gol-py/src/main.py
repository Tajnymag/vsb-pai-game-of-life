import multiprocessing
from multiprocessing import Pool
from multiprocessing.sharedctypes import Array, RawArray

import numpy as np
import pygame

from numpy.typing import NDArray

from gol import evolve_cells
from src.patterns import detect_patterns
from patterns import copy_to_board, n_glider_loop

BOARD_WIDTH = 128
BOARD_HEIGHT = 128


def render_board(board: NDArray[bool], screen: pygame.Surface):
    screen.fill("white")
    for (cell_x, cell_y), cell_alive in np.ndenumerate(board):
        if cell_alive:
            screen.set_at((cell_x, cell_y), "black")
    pygame.display.flip()


if __name__ == "__main__":
    board = np.zeros((BOARD_WIDTH, BOARD_HEIGHT), dtype=bool)

    pygame.init()
    flags = pygame.RESIZABLE | pygame.SCALED
    screen = pygame.display.set_mode((BOARD_WIDTH, BOARD_HEIGHT), flags)
    screen.fill("white")
    pygame.display.set_caption("VŠB PAI - Game of Life")
    clock = pygame.time.Clock()

    seed = n_glider_loop
    copy_to_board((BOARD_WIDTH // 2) - (seed.width // 2), (BOARD_HEIGHT // 2) - (seed.height // 2), seed, board)

    pattern_output_buffer = Array("i", BOARD_WIDTH * BOARD_HEIGHT)

    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
                pygame.quit()

        board = evolve_cells(board)
        #detect_patterns(board, pattern_output_buffer)

        render_board(board, screen)

        clock.tick(60)
