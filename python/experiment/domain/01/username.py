class UserName():
    def __init__(self, user_name: str):
        if len(user_name) < 3:
            raise ValueError("ユーザ名は3文字以上を設定してください")
        if len(user_name) > 20:
            raise ValueError("ユーザ名は20文字以下を設定してください")

        self.user_name = user_name
