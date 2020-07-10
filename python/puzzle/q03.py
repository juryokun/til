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



def main_another():
    MAX_CNT = 101
    card_list = [[i, False] for i in range(1, MAX_CNT)]

    for start in range(2, MAX_CNT, 1):
        func = gen_turn_over_func(start)
        map(func, card_list)

    for card in filter(lambda x: x[1] == False, card_list):
        print(card[0])


def gen_turn_over_func(number):
    def turn_over(card):
        if card[0] % number == 0:
            card[1] = not card[1]
        return card
    return turn_over



main()
main_another()