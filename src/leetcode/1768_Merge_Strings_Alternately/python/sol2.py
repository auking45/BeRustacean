#!/usr/bin/env python3

def mergeAlternately(word1: str, word2: str) -> str:
    merged = [a + b for a, b in zip(word1, word2)]
    return "".join(merged) + word1[len(word2):] + word2[len(word1):]

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
