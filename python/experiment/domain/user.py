from username import UserName
from userid import UserId


class User():

    def __init__(self, user_name: UserName):
        self.user_name = user_name
        self.user_id = UserId(user_name)
