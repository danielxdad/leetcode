# https://leetcode.com/problems/zigzag-conversion/
import itertools


def solution(s: str = '', numRows: int = 0) -> str:
    if len(s) < 3 or numRows < 2:
        return s

    window = numRows * 3 - 2
    diag_len = window - (numRows * 2)
    chunks = []

    # print(window, diag_len)
    # print('=' * 20)

    for wi in range(0, len(s), numRows + diag_len):
        chunk = s[wi: wi + window]

        # print(chunk)
        # print(col_1)
        # print(diag)
        # print(col_2)
        # print('=' * 20)

        col_1 = chunk[0: numRows]
        if not chunks:
            chunks.append(col_1)
        else:
            if chunks[-1] != col_1:
                chunks.append(col_1)

        diag = chunk[numRows: numRows + diag_len]
        for i, ch in enumerate(diag):
            col = ' ' * (numRows - (i + 2))
            col += ch
            col += ' ' * (i + 1)
            chunks.append(col)

        col_2 = chunk[numRows + diag_len:]
        chunks.append(col_2)

    result = ''
    for line in itertools.zip_longest(*chunks, fillvalue=''):
        result += ''.join(line).replace(' ' , '')

    # print(chunks)

    return result



if __name__ == '__main__':
    result = solution('PAYPALISHIRING', 3)

    print(result)

