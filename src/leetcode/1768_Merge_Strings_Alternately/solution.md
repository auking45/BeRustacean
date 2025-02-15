## 🔹 문제 요약

두 개의 문자열 word1과 word2가 주어졌을 때, 두 문자열의 문자를 번갈아 가며 결합하는 문제야.

- 예제 1:
    ```python
    word1 = "abc", word2 = "pqr"
    ```
    → 결과: "apbqcr"

- 예제 2:
    ```python
    word1 = "ab", word2 = "pqrs"
    ```
    → 결과: "apbqrs" (더 긴 word2의 나머지 부분이 뒤에 붙음)

## 🔹 풀이 방법

1️⃣ 투 포인터 사용 → 두 문자열을 동시에 순회하면서 결과 문자열을 만든다.  
2️⃣ zip 활용 → zip()을 사용하면 두 문자열을 동시에 처리하기 쉽다.  
3️⃣ itertools 활용 → zip_longest()로 길이가 다른 경우도 깔끔하게 처리할 수 있다.  

## 🔹 Python 코드 3가지 방법
### ✅ 1. 투 포인터 (기본적인 반복문)
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

**✔️ 시간 복잡도: O(N)**  
**✔️ 공간 복잡도: O(N) (결과 문자열 저장)**

### ✅ 2. zip() 활용 (Pythonic한 방법)
```python
def mergeAlternately(word1: str, word2: str) -> str:
    merged = [a + b for a, b in zip(word1, word2)]
    return "".join(merged) + word1[len(word2):] + word2[len(word1):]
```

#### ✔️ zip() 사용 → 두 개의 문자열을 동시에 묶어서 처리
#### ✔️ 남은 부분 추가 → word1[len(word2):] + word2[len(word1):]

### ✅ 3. itertools.zip_longest() 활용 (더 직관적)
```python
from itertools import zip_longest

def mergeAlternately(word1: str, word2: str) -> str:
        
```

#### ✔️ `zip_longest()`를 쓰면 길이가 다른 경우도 자동 처리
#### ✔️ a or b → None 대신 빈 문자열로 처리

## 🔹 어떤 풀이가 가장 좋을까?
- 투 포인터 풀이: 직관적이고 C/C++ 스타일에 가까워
- zip 활용: Pythonic하고 가독성이 좋음
- itertools 활용: 가장 깔끔한 방법 (추천!)
