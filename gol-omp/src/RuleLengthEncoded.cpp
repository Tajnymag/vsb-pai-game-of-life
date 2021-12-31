#include "RuleLengthEncoded.h"

#include <regex>
#include <iostream>
#include <cassert>

RuleLengthEncoded::RuleLengthEncoded(const std::string &string_format) {
    std::istringstream digestable_input(string_format);

    this->width = 0;
    this->height = 0;
    this->rule = "R23/S3";

    bool header_set = false;
    std::regex header_regex(R"(x\s*=\s*(\d+)\s*,\s*y\s*=\s*(\d+)\s*(?:,\s*rule\s*=\s*([\w\/]+))?)");

    std::string encoded_pattern;
    std::string line;
    while (std::getline(digestable_input, line)) {
        if (line.starts_with("#C")) {
            this->comment += std::regex_replace(line, std::regex("^#C"), "");
        } else if (line.starts_with("#N")) {
            this->name = std::regex_replace(line, std::regex("^#N"), "");
        } else if (line.starts_with("#O")) {
            this->createdBy = std::regex_replace(line, std::regex("^#O"), "");
        } else if (line.starts_with("#r")) {
            this->rule = std::regex_replace(line, std::regex("^#r"), "");
        } else if (line.starts_with("#")) {
            std::cout << "Ignoring comment line: " << line << std::endl;
        } else if (std::regex_search(line, header_regex)) {
            std::smatch matches;
            std::regex_match(line, matches, header_regex);

            auto width_string = matches[1];
            auto height_string = matches[2];

            if (matches.size() > 3) {
                this->rule = matches[4];
            }

            this->width = std::stoi(width_string);
            this->height = std::stoi(height_string);
            header_set = true;
        } else if (header_set) {
            encoded_pattern += line;
        } else {
            throw "Encountered an unexpectedly formatted line!\n" + line;
        }
    }

    char last_token = 'x';
    int tag_count = 1;
    std::vector<int> decoded_pattern_line;

    for (const char token: encoded_pattern) {
        if (std::isdigit(token)) {
            if (std::isdigit(last_token)) {
                tag_count = tag_count * 10 + (token - '0');
            } else {
                tag_count = token - '0';
            }
        } else if (token == 'o' || token == 'b') {
            for (int i = 0; i < tag_count; ++i) {
                decoded_pattern_line.push_back(token == 'o');
            }
            tag_count = 1;
        } else if (token == '$') {
            for (int i = 0; i < tag_count; ++i) {
                for (const bool decoded_token: decoded_pattern_line) {
                    this->data.emplace_back(decoded_token);
                }
                int width_difference = this->width - decoded_pattern_line.size();
                for (int j = 0; j < width_difference; ++j) {
                    this->data.emplace_back(false);
                }
            }
            decoded_pattern_line.clear();
            tag_count = 1;
        } else if (token == '!') {
            if (last_token != '$') {
                for (const bool decoded_token: decoded_pattern_line) {
                    this->data.emplace_back(decoded_token);
                }
                int width_difference = this->width - decoded_pattern_line.size();
                for (int j = 0; j < width_difference; ++j) {
                    this->data.emplace_back(false);
                }
            }
        } else {
            std::string token_str;
            token_str += token;
            throw "Unexpected token encountered in line:\n" + line;
        }

        last_token = token;
    }

    assert(("Encoded pattern does not match specified dimensions!", this->data.size() == this->width * this->height));
}
