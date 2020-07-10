# -*- coding: utf-8 -*-
import sys

def main():
    MAX_CNT = 101
    card_list = [[i, False] for i in range(1, MAX_CNT)]

    for start in range(2, MAX_CNT, 1):
        vals = [start for i in range(len(card_list))]
        map(turn_over, card_list, vals)

    for card in filter(lambda x: x[1] == False, card_list):
        print(card[0])

def turn_over(card, number):
    if card[0] % number == 0:
        card[1] = not card[1]
    return card

main()