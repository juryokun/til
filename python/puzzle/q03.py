# -*- coding: utf-8 -*-
import sys

def main():
    MAX_CNT = 101
    card_list = [[i, False] for i in range(1, MAX_CNT)]

    for start in range(2, MAX_CNT, 1):
        inc = start
        for index in range(start, MAX_CNT, inc):
            card_list[index-1][1] = not card_list[index-1][1]

    for card in filter(lambda x: x[1] == False, card_list):
        print(card[0])

main()