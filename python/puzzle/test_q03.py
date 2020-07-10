# -*- coding: utf-8 -*-

import unittest

from q03 import turn_over
from q03 import gen_turn_over_func

class TestTurnOver(unittest.TestCase):
    def test_turn_over(self):
        data = [
            [[2, False], 2, [2, True]],
            [[9, False], 3, [9, True]],
            [[29, False], 5, [29, False]],
            [[44, True], 4, [44, False]],
        ]
        for target in data:
            result = turn_over(target[0], target[1])
            if result != target[2]:
                print(target[0])
            self.assertEqual(result, target[2])

    def test_gen_turn_over_func(self):
        data = [
            [[2, False], 2, [2, True]],
            [[9, False], 3, [9, True]],
            [[29, False], 5, [29, False]],
            [[44, True], 4, [44, False]],
        ]
        for target in data:
            func = gen_turn_over_func(target[1])
            result = func(target[0])
            if result != target[2]:
                print(target[0])
            self.assertEqual(result, target[2])

if __name__ == "__main__":
    unittest.main()