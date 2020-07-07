
# -*- coding: utf-8 -*-

def rpn(input):

    operators = ["+", "-", "/", "*"]
    stack = []

    for x in input.split(" "):
        if x not in operators:
            stack.append(x)
            continue
        stack.append(calcurate(stack, x))
    return stack.pop()

def calcurate(stack, op):
    y = int(stack.pop())
    x = int(stack.pop())

    if op == "+":
        return x + y
    elif op == "-":
        return x - y
    elif op == "/":
        return float(x) / y
    elif op == "*":
        return x * y

def test():
    result = rpn("12 2 5 * -")
    assert result == 2, 'False'

if __name__ == '__main__':
    import sys
    print(rpn(sys.argv[1]))
    test()


