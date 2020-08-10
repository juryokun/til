def main():
    max_boy = 20+1
    max_girl = 10+1
    arr = [[0 for j in range(max_girl)] for i in range(max_boy)]
    arr[0][0] = 1
    for i in range(max_boy):
        for j in range(max_girl):
            if i != j and max_boy - i != max_girl - j:
                if i > 0:
                    arr[i][j] += arr[i-1][j]
                if j > 0:
                    arr[i][j] += arr[i][j-1]
    print(arr[max_boy-2][max_girl-1]+arr[max_boy-1][max_girl-2])

main()