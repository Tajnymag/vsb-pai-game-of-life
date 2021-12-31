#include <vector>

#define SDL_MAIN_HANDLED

#include "SDL2/SDL.h"

#include "patterns.h"
#include "RuleLengthEncoded.h"
#include "utils.h"

const int DEFAULT_BOARD_WIDTH = 128;
const int DEFAULT_BOARD_HEIGHT = 128;

const int CELL_SIZE = 8;

template<typename T>
void resize_board(std::vector<T> &board, const int old_width, const int old_height, const int new_width,
                  const int new_height) {
    std::vector old_board(board);

    board.resize(new_width * new_height, 0);

    for (int old_board_x = 0; old_board_x < old_width; ++old_board_x) {
        for (int old_board_y = 0; old_board_y < old_height; ++old_board_y) {
            auto new_board_index = to_coordinate_1D(old_board_x, old_board_y, new_width);
            auto old_board_index = to_coordinate_1D(old_board_x, old_board_y, old_width);

            if (old_board_x >= new_width || old_board_y >= new_height) {
                continue;
            }

            board[new_board_index] = old_board[old_board_index];
        }
    }
}

void render_board(SDL_Renderer *renderer, const std::vector<bool> &board, const std::vector<uint8_t> &pattern_board,
                  const int board_width) {
    SDL_SetRenderDrawColor(renderer, 255, 255, 255, 0);
    SDL_RenderClear(renderer);

    for (int cell_index = 0; cell_index < board.size(); ++cell_index) {
        bool cell_alive = board[cell_index];
        auto cell_pos = to_coordinate_2D(cell_index, board_width);

        if (cell_alive) {
            auto cell_rect = SDL_Rect{cell_pos.x * CELL_SIZE, cell_pos.y * CELL_SIZE, CELL_SIZE, CELL_SIZE};

            auto pattern_color_index = pattern_board[cell_index];
            auto pattern_color = PATTERN_COLORS[pattern_color_index];

            SDL_SetRenderDrawColor(renderer, pattern_color.r, pattern_color.g, pattern_color.b, pattern_color.a);
            SDL_RenderFillRect(renderer, &cell_rect);
        }
    }
}

int
num_of_neighbors(const int cell_index, const std::vector<bool> &board, const int board_width, const int board_height) {
    auto cell_pos = to_coordinate_2D(cell_index, board_width);
    int neighbors_count = 0;

    const auto neighbor_offsets = {
            Coordinates{-1, -1},
            Coordinates{0, -1},
            Coordinates{1, -1},
            Coordinates{-1, 0},
            Coordinates{1, 0},
            Coordinates{-1, 1},
            Coordinates{0, 1},
            Coordinates{1, 1}
    };

    for (const auto &offset: neighbor_offsets) {
        int neighbor_x = cell_pos.x + offset.x;
        int neighbor_y = cell_pos.y + offset.y;

        if (neighbor_x < 0 || neighbor_x >= board_width) {
            continue;
        }
        if (neighbor_y < 0 || neighbor_y >= board_height) {
            continue;
        }

        int neighbor_index = to_coordinate_1D(neighbor_x, neighbor_y, board_width);

        neighbors_count += board[neighbor_index];
    }

    return neighbors_count;
}

void evolve_cells(const std::vector<bool> &board, std::vector<bool> &output_board, const int board_width,
                  const int board_height) {
#pragma omp parallel for default(none) shared(board, output_board, board_width, board_height)
    for (int cell_index = 0; cell_index < (board_width * board_height); ++cell_index) {
        bool cell_alive = board[cell_index];
        int neighbors_count = num_of_neighbors(cell_index, board, board_width, board_height);

        output_board[cell_index] = (cell_alive && (neighbors_count == 2 || neighbors_count == 3)) ||
                                   (not cell_alive && (neighbors_count == 3));
    }
}

int main(int argc, char *argv[]) {
    SDL_Init(SDL_INIT_VIDEO);
    SDL_Window *window = SDL_CreateWindow(
            "VŠB PAI - Game of Life",
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            DEFAULT_BOARD_WIDTH * CELL_SIZE,
            DEFAULT_BOARD_HEIGHT * CELL_SIZE,
            SDL_WINDOW_SHOWN | SDL_WINDOW_ALLOW_HIGHDPI | SDL_WINDOW_RESIZABLE
    );

    SDL_Renderer *renderer = SDL_CreateRenderer(
            window,
            -1,
            SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC
    );

    int board_width = DEFAULT_BOARD_WIDTH;
    int board_height = DEFAULT_BOARD_HEIGHT;

    auto current_board = std::vector<bool>(board_width * board_height, false);
    auto next_board = std::vector<bool>(board_width * board_height, false);
    auto pattern_board = std::vector<uint8_t>(board_width * board_height, 0);

    auto seed = (argc > 1) ? load_pattern_from_file(argv[1]) : GLIDER_LOOP;

    copy_to_board(board_width / 2 - seed.width / 2, board_height / 2 - seed.height / 2, seed, current_board,
                  board_width);

    SDL_SetRenderDrawColor(renderer, 255, 255, 255, 0);
    SDL_RenderClear(renderer);

    bool quit = false;

    SDL_Event event;
    while (!quit) {
        while (SDL_PollEvent(&event)) {
            switch (event.type) {
                case SDL_QUIT:
                    quit = true;
                    break;
                case SDL_WINDOWEVENT:
                    switch (event.window.event) {
                        case SDL_WINDOWEVENT_RESIZED: {
                            int new_width = event.window.data1 / CELL_SIZE;
                            int new_height = event.window.data2 / CELL_SIZE;

                            resize_board(current_board, board_width, board_height, new_width, new_height);
                            resize_board(next_board, board_width, board_height, new_width, new_height);

                            pattern_board.resize(new_width * new_height, 0);

                            board_width = new_width;
                            board_height = new_height;
                            break;
                        }
                        default:
                            break;
                    }
                    break;
                default:
                    break;
            }
        }

        evolve_cells(current_board, next_board, board_width, board_height);

        std::swap(current_board, next_board);

        std::fill(pattern_board.begin(), pattern_board.end(), 0);
        detect_patterns(current_board, pattern_board, board_width, board_height);

        render_board(renderer, current_board, pattern_board, board_width);

        SDL_RenderPresent(renderer);
        SDL_Delay(1000 / 60);
    }

    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
    return 0;
}
