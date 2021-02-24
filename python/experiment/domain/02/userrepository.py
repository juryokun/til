from iuserrepository import IUserRepository
from user import User
from username import UserName


class UserRepository(IUserRepository):
    def __init__(self):
        pass

    def save(self, user: User):
        # クエリ
        pass

    def find(self, name: UserName) -> User:
        pass

    def exists(self, user: User) -> bool:
        pass
