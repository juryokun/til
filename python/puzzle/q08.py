# -*- coding: utf-8 -*-

def move(history, max):
    if len(history) == max + 1:
        return 1

    cnt = 0
    loc = history[-1]
    for i in [[0, 1], [0, -1], [1, 0], [-1, 0]]:
        next = [i[0]+loc[0], i[1]+loc[1]]
        if next not in history:
            cnt += move(history + [next], max)
        else:
            continue

    return cnt

if __name__ == "__main__":
    print(move([[0,0]], 12))