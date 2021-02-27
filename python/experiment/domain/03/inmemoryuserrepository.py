from iuserrepository import IUserRepository
from user import User
from username import UserName


class InMemoryUserRepository(IUserRepository):

    def __init__(self):
        test_user = User(UserName("yamada"))
        self.store = {test_user.id: test_user}

    def save(self, user: User):
        self.store[user.id] = user

    def find(self, name: UserName) -> User:
        for val in self.store.values():
            if val.name.user_name == name.user_name:
                return val
        return None

    def delete(self, user: User):
        pass

    def exists(self, user: User) -> bool:
        return user.id in self.store.keys()
