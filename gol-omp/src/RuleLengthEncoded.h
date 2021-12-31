#ifndef GOL_OMP_RULELENGTHENCODED_H
#define GOL_OMP_RULELENGTHENCODED_H

#include <string>
#include <vector>
#include <string>

class RuleLengthEncoded {
public:
    std::string name;
    std::string comment;
    std::string createdBy;
    std::string rule;

    int width;
    int height;
    std::vector<bool> data;

    RuleLengthEncoded(const std::string &string_format);
};


#endif //GOL_OMP_RULELENGTHENCODED_H
