# -*- coding: utf-8 -*-

import unittest

from q01 import is_palindrome

class TestIsPalindrome(unittest.TestCase):
    def test_is_palindrome(self):
        data = [
            [str(10), False],
            [str(88), True],
            [str(800), False],
            [str(797), True],
            [str(12345654321), True],
            [format(4, 'b'), False],
            [format(5, 'b'), True],
            [format(512, 'o'), False],
            [format(513, 'o'), True],
        ]
        for target in data:
            result = is_palindrome(target[0])
            if result != target[1]:
                print(target[0])
            self.assertEqual(result, target[1])

if __name__ == "__main__":
    unittest.main()