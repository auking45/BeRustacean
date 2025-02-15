# 🚀 Merge Strings Alternately - Interview Explanation

## 1. 📝 Problem Understanding
The problem gives us two strings, `word1` and `word2`. Our goal is to merge them alternately, character by character. If one string is longer, we append the remaining characters at the end.

### 📌 Example Walkthrough
```python
word1 = "abc", word2 = "pqr"
# Output: "apbqcr"

word1 = "ab", word2 = "pqrs"
# Output: "apbqrs"
```

## 2. 🔍 Approach
There are multiple ways to solve this problem efficiently. I'll explain a simple iterative approach using two pointers.

1️⃣ Use two pointers (`i` and `j`) to iterate through both strings simultaneously.  
2️⃣ Add characters from `word1` and `word2` alternately to a result list.  
3️⃣ If one string is longer, append the remaining part at the end.  

## 3. 💻 Code Explanation
```python
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
```
### 🧐 Explanation
We initialize two pointers `i` and `j` at 0. We iterate through both strings until we reach the end of both. In each iteration, we append one character from each string alternately. If one string is longer, the remaining characters are appended at the end.

## 4. ⏳ Time Complexity Analysis
Since we traverse both strings once, the time complexity is **O(N + M)**, where N and M are the lengths of the two strings. The space complexity is also **O(N + M)** because we store the result in a list before converting it to a string.

## 5. 🔄 Alternative Approaches
We can also use Python's built-in `zip()` function or `itertools.zip_longest()` for a more Pythonic approach.

```python
from itertools import zip_longest

def mergeAlternately(word1: str, word2: str) -> str:
    return "".join(a + b if a and b else (a or b) for a, b in zip_longest(word1, word2, fillvalue=""))
```

### 🧐 Explanation
This approach is more concise and uses `zip_longest()` to handle different string lengths automatically.

## 6. 🎯 Conclusion
This method ensures an efficient solution with **O(N + M) time complexity**, and it is easy to implement. The alternative approach using `zip_longest()` makes the solution more concise and readable. 😊
