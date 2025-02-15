#!/usr/bin/env python3

def mergeAlternately(word1: str, word2: str) -> str:
    res = []
    i, j = 0, 0
    while i < len(word1) or j < len(word2):
        if i < len(word1):
            res.append(word1[i])
            i += 1
        if j < len(word2):
            res.append(word2[j])
            j += 1
    return "".join(res)

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
