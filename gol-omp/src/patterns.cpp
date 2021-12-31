#include <SDL_log.h>
#include "patterns.h"
#include "utils.h"

RuleLengthEncoded load_pattern_from_file(const char *file_path) {
    std::ifstream pattern_file(file_path);
    std::stringstream buffer;

    buffer << pattern_file.rdbuf();

    return RuleLengthEncoded(buffer.str());
}

void detect_patterns(const std::vector<bool> &board, std::vector<uint8_t> &pattern_board, int board_width,
                     int board_height) {
#pragma omp parallel for default(none) shared(board, pattern_board, board_width, board_height, SEARCHABLE_PATTERNS)
    for (int i = 0; i < board.size(); ++i) {
        int pattern_start = i;

        for (int pattern_id = 0; pattern_id < SEARCHABLE_PATTERNS.size(); ++pattern_id) {
            auto pattern = SEARCHABLE_PATTERNS[pattern_id];

            bool skip_pattern = false;

            for (int pattern_cell_index = 0; pattern_cell_index < pattern.data.size(); ++pattern_cell_index) {
                bool pattern_cell_value = pattern.data[pattern_cell_index];
                auto pattern_cell_coords = to_coordinate_2D(pattern_cell_index, pattern.width);

                int cell_index =
                        pattern_start + to_coordinate_1D(pattern_cell_coords.x, pattern_cell_coords.y, board_width);

                if (cell_index >= board.size()) {
                    skip_pattern = true;
                    break;
                }

                bool cell_value = board[cell_index];

                if (cell_value != pattern_cell_value) {
                    skip_pattern = true;
                    break;
                }

                if (pattern_board[cell_index] > 0) {
                    skip_pattern = true;
                    break;
                }
            }

            if (skip_pattern) continue;

            for (int pattern_cell_index = 0; pattern_cell_index < pattern.data.size(); ++pattern_cell_index) {
                auto pattern_cell_coords = to_coordinate_2D(pattern_cell_index, pattern.width);
                auto pattern_start_coords = to_coordinate_2D(pattern_start, board_width);
                int cell_index = to_coordinate_1D(pattern_cell_coords.x + pattern_start_coords.x,
                                                  pattern_cell_coords.y + pattern_start_coords.y, board_width);

                pattern_board[cell_index] = pattern_id + 1;
            }
        }
    }
}

void copy_to_board(const int x, const int y, const RuleLengthEncoded &pattern, std::vector<bool> &board,
                   const int board_width) {
    for (int pattern_y = 0; pattern_y < pattern.height; ++pattern_y) {
        for (int pattern_x = 0; pattern_x < pattern.width; ++pattern_x) {
            int cell_index = to_coordinate_1D(pattern_x + x, pattern_y + y, board_width);
            int pattern_cell_index = to_coordinate_1D(pattern_x, pattern_y, pattern.width);

            if (pattern_x > board_width || (pattern_x * pattern_y) > board.size()) {
                continue;
            }

            board[cell_index] = pattern.data[pattern_cell_index];
        }
    }
}