def weekAndTime(day, hour):
    if day > 7:
        return 0

    print('day={day}, hour={hour}'.format(day=day, hour=hour))
    if hour >= 24:
        weekAndTime(day+1, 1)
    else:
        weekAndTime(day, hour+1)

weekAndTime(1, 1)