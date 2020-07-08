# -*- coding: utf-8 -*-

import unittest

from rpn import rpn

class TestRpn(unittest.TestCase):
    def test_rpn(self):
        data = [
            ["1 2 + 4 -", -1],
        ]
        for target in data:
            result = rpn(target[0])
            if result != target[1]:
                print(target[0])
            self.assertEqual(result, target[1])

    def test_exception_rpn(self):
        data = [
            ["1 1 - 0 /", ZeroDivisionError, "ゼロで割ることはできません"],
            ["a 12 -", ValueError, "値が正しくありません"],
            ["1 12 1", Exception, "式が正しくありません"],
            ["1 -", IndexError, "式が正しくありません"],
        ]
        for target in data:
            with self.assertRaises(target[1], msg=target[2]):
                rpn(target[0])

if __name__ == "__main__":
    unittest.main()