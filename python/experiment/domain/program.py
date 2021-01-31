from user import User
from username import UserName
from iuserrepository import IUserRepository
from userservice import UserService


class Program():

    def __init__(self, user_repository: IUserRepository):
        self._user_repository = user_repository

    def create_user(self, user_name: str):
        user = User(UserName(user_name))

        user_service = UserService(self._user_repository)
        if user_service.exists(user):
            raise(user + "は既に存在しています。")

        user_service.save(user)
