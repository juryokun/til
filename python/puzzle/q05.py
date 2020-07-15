# -*- coding: utf-8 -*-

import copy

def main(num):
    cnt = 0
    for a in range(101):
        for b in range(21):
            for c in range(11):
                for d in range(3):
                    summary = 10 * a + 50 * b + 100 * c + 500 * d
                    coin = a+b+c+d
                    if (summary == num) and (coin <= 15):
                        cnt += 1
                        print('a={a}, b={b}, c={c}, d={d}'.format(a=a,b=b,c=c,d=d))
    print(cnt)

rel = 0
def change(target, coins, limit, count=0):
    if target == 0 and count <= limit and count > 0:
        global rel
        rel += 1
        return 0
    if target < 0:
        return 0
    if count > limit:
        return 0
    if len(coins) > 0:
        coin = coins.pop()
        for num in range(limit+1):
            change(target-coin*num, copy.copy(coins), limit, count+num)

if __name__ == "__main__":
    main(1000)
    change(1000, [10, 50, 100, 500], 15)
    print(rel)