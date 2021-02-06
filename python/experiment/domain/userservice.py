from iuserrepository import IUserRepository
from user import User


class UserService():
    def __init__(self, user_repository: IUserRepository):
        self._user_repository = user_repository

    def exists(self, user: User) -> bool:
        return self._user_repository.exists(user)

    def save(self, user: User):
        self._user_repository.save(user)
