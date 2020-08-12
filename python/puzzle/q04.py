# -*- coding: utf-8 -*-

# 回答1: ループでやる
def ans1(poll, people_num):
    target_list = []
    target_list.append(poll)

    cnt = 0
    # while len(list(filter(lambda x: x>1, target_list))) > 0:
    while len([x for x in target_list if x>1]) > 0:
        stack = []
        for i in range(people_num):
            target = select_target(target_list)
            if target == False:
                break
            del target_list[target[0]]
            stack.append(target[1])

        for target in stack:
            left = int(target/2)
            right = target - left
            target_list.append(left)
            target_list.append(right)
        cnt += 1
    return cnt

def select_target(targets):
    for j, target in enumerate(targets):
        if target > 1:
            return [j, target]
    return False


# 回答2: 再帰でやる
def ans2(poll, num, split):
    if split >= poll:
        return 0
    elif split < num:
        return 1 + ans2(poll, num, split * 2)
    else:
        return 1 + ans2(poll, num, split + num)


if __name__ == "__main__":
    print(ans1(20, 3))
    print(ans1(100, 5))
    print(ans2(20, 3, 1))
    print(ans2(100, 5, 1))