
# -*- coding: utf-8 -*-
import sys

# TODO: python rpn.py "1 2 + 1"
def rpn(input):

    operators = ["+", "-", "/", "*"]
    stack = []

    for z in input.split(" "):
        if z not in operators:
            try:
                stack.append(int(z))
            except ValueError as e:
                raise ValueError("値が正しくありません")
            continue
        stack.append(calcurate(stack, z))

    return stack.pop()

def calcurate(stack, op):
    try:
        y = stack.pop()
        x = stack.pop()
    except IndexError as e:
        raise IndexError("式が正しくありません")

    try:
        if op == "+":
            return x + y
        elif op == "-":
            return x - y
        elif op == "/":
            return float(x) / y
        elif op == "*":
            return x * y
    except ZeroDivisionError as e:
        raise ZeroDivisionError("ゼロで割ることはできません")

def test():
    result = rpn("12 2 5 * -")
    assert result == 2, 'False'

if __name__ == '__main__':
    if len(sys.argv) == 2:
        try:
            print(rpn(sys.argv[1]))
        except Exception as e:
            print(e)
        test()
    else:
        print("引数が正しくありません")


