# -*- coding: utf-8 -*-

print('好きな文字を入力してください')
text = input()

transformedInt = int(text.encode('utf-8').hex(), 16)
print(transformedInt)

print('ＨＰ：', transformedInt)
print('攻撃力：', transformedInt / 3)
print('防御力：', transformedInt / 4)
