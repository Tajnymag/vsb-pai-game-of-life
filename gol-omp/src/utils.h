#ifndef GOL_OMP_UTILS_H
#define GOL_OMP_UTILS_H

struct Coordinates {
    int x;
    int y;
};

int to_coordinate_1D(Coordinates coordinates, int board_width);

int to_coordinate_1D(int x, int y, int board_width);

Coordinates to_coordinate_2D(int i, int board_width);

#endif //GOL_OMP_UTILS_H
