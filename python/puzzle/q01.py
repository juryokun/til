# -*- coding: utf-8 -*-

def palindrome():
    i = 11
    while True:
        if is_palindrome(str(i)) and is_palindrome(format(i, 'b')) and is_palindrome(format(i, 'o')):
            return i
        i += 1

def is_palindrome(target):
    if target == ''.join(reversed(target)):
        return True
    return False

def main():
    ans = palindrome()
    print(ans)

main()