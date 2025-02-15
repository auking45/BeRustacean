#!/usr/bin/env python3

from itertools import zip_longest

def mergeAlternately(word1: str, word2: str) -> str:
    return "".join(a + b if a and b else (a or b) for a, b in zip_longest(word1, word2, fillvalue=""))

if __name__ == "__main__":
    inputs = [
        ("abc", "pqr"),
        ("ab", "pqrs"),
        ("abcd", "pq")
    ]
    outputs = [
        "apbqcr",
        "apbqrs",
        "apbqcd"
    ]

    for i, input in enumerate(inputs):
        output = outputs[i]
        ret = mergeAlternately(*input)

        print(f"input: {input} | output: {output} | ret: {ret}")
        assert(ret == output)
