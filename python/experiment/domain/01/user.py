from username import UserName
from userid import UserId


class User():

    def __init__(self, user_name: UserName = None, user_id: UserId = None):
        if user_name != None:
            self.name = user_name
        else:
            raise AttributeError("ユーザ名が入力されてません")

        if user_id != None:
            self.id = user_id
        else:
            self.id = UserId(user_name.user_name)

    def change_name(self, name: UserName):
        self.name = name
