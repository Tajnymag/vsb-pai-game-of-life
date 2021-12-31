#include "utils.h"

int to_coordinate_1D(const Coordinates &coordinates, const int board_width) {
    return board_width * coordinates.y + coordinates.x;
}

int to_coordinate_1D(const int x, const int y, const int board_width) {
    return board_width * y + x;
}

Coordinates to_coordinate_2D(const int i, const int board_width) {
    int x = i % board_width;
    int y = i / board_width;

    return Coordinates{x, y};
}