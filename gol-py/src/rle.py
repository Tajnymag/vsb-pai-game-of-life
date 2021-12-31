import re
from typing import List, Any

import numpy
import numpy as np
from numpy.typing import NDArray


class RuleLengthEncoded:
    name: str = "unknown"
    comment: str = ""
    createdBy: str = ""
    rule: str = "23/3"

    width: int = 0
    height: int = 0
    data: NDArray[bool] = np.array([])

    def __init__(self, string_format: str):
        lines = (line.rstrip() for line in string_format.splitlines())
        lines = (line for line in lines if line)

        encoded_pattern = ""

        header_set = False
        header_regex = re.compile(r'x\s*=\s*(?P<width>\d+)\s*,\s*y\s*=\s*(?P<height>\d+)\s*(?:,\s*rule\s*=\s*(?P<rule>[\w\/]+))?')

        for line in lines:
            if line.startswith("#C") or line.startswith("#c"):
                self.comment += re.sub(r'^#C', "", line, flags=re.I)
            elif line.startswith("#N"):
                self.name = re.sub(r'^#N', "", line, flags=re.I)
            elif line.startswith("#O"):
                self.createdBy = re.sub(r'^#O', "", line, flags=re.I)
            elif line.startswith("#r"):
                self.rule = re.sub(r'^#r', "", line, flags=re.I)
            elif line.startswith("#"):
                print(f"Ignoring comment line: {line}")
            elif header_regex.match(line):
                match = header_regex.match(line)

                width = int(match.group("width"))
                height = int(match.group("height"))
                rule = match.group("rule") or "23/3"

                self.width = width
                self.height = height
                self.rule = rule
                header_set = True
            elif header_set:
                encoded_pattern += line.rstrip()
            else:
                raise Exception(f"Encountered an unexpectedly formatted line!\n{line}")

        last_token = ""
        tag_count = 1
        decoded_pattern_line: List[bool] = []
        data: List[List[bool]] = []

        i = -1
        for token in encoded_pattern:
            i += 1
            if token.isnumeric():
                if last_token.isnumeric():
                    tag_count = tag_count * 10 + int(token)
                else:
                    tag_count = int(token)
            elif token == "o" or token == "b":
                for _ in range(0, tag_count):
                    decoded_pattern_line.append(token == "o")
                tag_count = 1
            elif token == "$":
                for _ in range(0, tag_count):
                    data.append(decoded_pattern_line)
                    decoded_pattern_line = []
                tag_count = 1
            elif token == "!":
                if last_token != "$":
                    data.append(decoded_pattern_line)
                    break
            else:
                raise Exception(f"Unexpected token {token} encountered on line {i} of pattern")

            last_token = token

        for pattern_line in data:
            while len(pattern_line) > self.width:
                pattern_line.pop()
            while len(pattern_line) < self.width:
                pattern_line.append(False)

        if len(data) != self.height:
            raise Exception("Encoded pattern does not match specified dimensions!")

        self.data = numpy.array(data)
