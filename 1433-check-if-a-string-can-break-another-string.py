# https://leetcode.com/problems/check-if-a-string-can-break-another-string/description/
# from itertools import permutations
# from math import factorial

# s1 = sorted(list(s1))
# s2 = sorted(list(s2))
# fact = factorial(len(s1))
# print("N!:", fact)

cases = [
    ["abc", "xya"],
    ["abe", "acd"],
    ["leetcodee", "interview"],
    ["mgldgsvnsgpdvm", "jqpaktmjafgkzs"],
]

for s1, s2 in cases:
    print('Case: s1="%s", s2="%s"' % (s1, s2))

    s1 = list(s1)
    s2 = list(s2)
    s1.sort()
    s2.sort()

    s1_small = True
    s2_small = True
    for i in range(len(s1)):
        cs1, cs2 = s1[i], s2[i]
        s1_small = s1_small and cs1 <= cs2
        s2_small = s2_small and cs2 <= cs1

    print(s1_small, s2_small)
    print()

# result = all(map(lambda pair: pair[0] <= pair[1], zip(s1, s2))) or all(
#     map(lambda pair: pair[0] >= pair[1], zip(s1, s2))
# )
# print(result)

# for perm_s1, perm_s2 in zip(permutations(s1), permutations(s2)):
#     diff = list(map(lambda pair: ord(pair[0]) - ord(pair[1]), zip(perm_s1, perm_s2)))
#     print("".join(perm_s1), " - ", "".join(perm_s2))
#     print(diff, sum(diff))
#     print("-" * 20)
#

# result = False
# for ps1, ps2 in zip(permutations(s1), permutations(s2)):
#     diff = list(map(lambda pair: ord(pair[0]) - ord(pair[1]), zip(ps1, ps2)))
#     if sum(diff) != 0 and (
#         all(map(lambda n: n >= 0, diff)) or all(map(lambda n: n <= 0, diff))
#     ):
#         print(list(ps1))
#         print(list(ps2))
#         print(diff)
#         result = True
#         break
#
# print(result)

# result = True
# for i, (ps1, ps2) in enumerate(zip(permutations(s1), permutations(s2))):
#     if i > 0 and i % 100 == 0:
#         print("%.4f\t\r" % (i / fact * 100), end="")
#
#     direction = None
#     for ch1, ch2 in zip(ps1, ps2):
#         # print(ch1, ch2)
#         diff = ord(ch1) - ord(ch2)
#
#         if diff == 0:
#             continue
#
#         if direction is None:
#             direction = diff / abs(diff)
#             continue
#
#         if direction != diff / abs(diff):
#             result = False
#             break
#     # print("-" * 20)
#
# print()
# print(result)
