with open("day05.txt", "r") as f:
    inputs = [s.strip() for s in f.readlines() if not s.isspace()]

# Got stuck debugging why my rust solution wasn't working on the full input
# so wrote this quickly to get the answer then use the output difference to debug.

# --> Turned out to be an edge-case when checking pairs at the end of a string - the
#     last pair was never considered
valids: list[str] = []
for test_string in inputs:
    pair_match: bool = False
    palindrome: bool = False

    for idx, _ in enumerate(test_string[2:]):
        pair = test_string[idx : idx + 2]
        if pair in test_string[idx + 2 :]:
            pair_match = True
            print(f"Pair duplicate found, str - {test_string}, pairs {pair}")
            break

    for idx in range(0, len(test_string) - 2):
        chunk = test_string[idx : idx + 3]
        if chunk == chunk[::-1]:
            palindrome = True
            print(f"Palindrome found, str - {test_string}, chars {chunk}")
            break

    if pair_match and palindrome:
        valids.append(test_string)

print(len(valids))
