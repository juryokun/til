# -*- coding: utf-8 -*-

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


if __name__ == "__main__":
    main(1000)